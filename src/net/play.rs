use std::path::PathBuf;

use crate::common::format_version_to_display;
use crate::input::{parse_gamestate, Game, InitOptions};
use crate::net::common::StreamType;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, UnixListener};
use tokio::runtime::Runtime;
use tokio::sync::broadcast;

pub fn block_on_play(props: StreamType) -> std::io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(play(props));
    Ok(())
}

pub async fn play(props: StreamType) {
    match parse_gamestate() {
        Ok(input) => {
            // prepare init_options
            let mut options_passthrough = input.options.clone();
            options_passthrough
                .features_with_version
                .insert("tcp-play-rust".to_string(), format_version_to_display());
            println!("{}\r", serde_json::to_string(&options_passthrough).unwrap());

            // prepare broadcast channel that will link between stdin and writing into tcp/unix sockets
            let (tx, _) = broadcast::channel::<Game>(10);

            // according to the input passed (either address:port, or filepath to unix socket)
            // spawn the correct server (tcp or unix socket)
            match props {
                StreamType::Tcp(bind_addr) => {
                    tokio::spawn(create_tcp_server(
                        bind_addr,
                        tx.clone(),
                        options_passthrough.clone(),
                    ));
                }
                StreamType::Socket(socket_path) => {
                    tokio::spawn(create_socket_server(
                        socket_path,
                        tx.clone(),
                        options_passthrough.clone(),
                    ));
                }
            }

            // reading stdin and broadcast to tx that will trigger writes on opened tcp/unix sockets
            brodcast_lines(input.lines, tx.clone()).unwrap();
        }
        Err(_) => todo!(),
    }
}

fn brodcast_lines(
    lines: Box<dyn Iterator<Item = Game>>,
    tx: broadcast::Sender<Game>,
) -> std::io::Result<()> {
    for parsed_line in lines {
        println!("{}\r", serde_json::to_string(&parsed_line).unwrap());
        // we don't care if there are clients to broadcast yet or not
        match tx.send(parsed_line) {
            Ok(lines) => {
                eprintln!("Clients active: {}\r", lines);
            }
            Err(_err) => {}
        }
    }
    Ok(())
}

async fn create_tcp_server(
    bind_addr: String,
    tx: tokio::sync::broadcast::Sender<Game>,
    init_options: InitOptions,
) -> std::io::Result<()> {
    match TcpListener::bind(&bind_addr).await {
        Ok(listener) => loop {
            match listener.accept().await {
                Ok((tcp_stream, _socket_addr)) => {
                    let tx = tx.clone();
                    let init_options = init_options.clone();
                    tokio::spawn(async move {
                        let _ = handle_client_task(tcp_stream, tx, init_options).await;
                    });
                }
                Err(e) => eprintln!("Couldn't get client {:?}\r", e),
            }
        },
        Err(err) => {
            if err.kind() == std::io::ErrorKind::AddrInUse {
                eprintln!("{} already in use.\r", bind_addr);
                std::process::exit(exitcode::IOERR);
            } else {
                eprintln!("Failed to launch TCP server.\r\n{:?}", err);
                std::process::exit(exitcode::IOERR);
            }
        }
    }
}

async fn create_socket_server(
    socket_path: PathBuf,
    tx: tokio::sync::broadcast::Sender<Game>,
    init_options: InitOptions,
) -> std::io::Result<()> {
    let listener = UnixListener::bind(socket_path)?;
    loop {
        match listener.accept().await {
            Ok((socket_stream, _socket_addr)) => {
                let tx = tx.clone();
                let init_options = init_options.clone();
                tokio::spawn(async move {
                    let _ = handle_client_task(socket_stream, tx, init_options).await;
                });
            }
            Err(e) => eprintln!("Couldn't get client {:?}\r", e),
        }
    }
}

async fn handle_client_task(
    mut stream: impl AsyncWriteExt + std::marker::Unpin,
    tx: tokio::sync::broadcast::Sender<Game>,
    init_options: InitOptions,
) -> std::io::Result<()> {
    eprintln!("before loop\r");
    let mut rx = tx.subscribe();
    stream
        .write_all(format!("{}\r\n", serde_json::to_string(&init_options).unwrap()).as_bytes())
        .await
        .unwrap();
    loop {
        tokio::select! {
            Ok(parsed_line) = rx.recv() => {
                // if we can't write to the tcp_stream, the connection must have been broken -> we exit handle_client_task
                // which will cleanup memory related to this connection (also unsubscribe to `tx` by dropping `rx`)
                match stream.write_all(format!("{}\r\n", serde_json::to_string(&parsed_line).unwrap()).as_bytes()).await {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

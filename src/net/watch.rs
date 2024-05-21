use crate::net::common::StreamType;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    runtime::Runtime,
};

pub fn block_on_watch(props: StreamType) -> std::io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(watch(props));
    Ok(())
}

pub async fn watch(props: StreamType) {
    match props {
        StreamType::Tcp(bind_addr) => handle_tcp_stream(bind_addr).await.unwrap(),
        StreamType::Socket(_socket_path) => todo!(),
    }
}

async fn handle_tcp_stream(bind_addr: String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(&bind_addr).await?;

    let mut buffer = [0; 1024];

    loop {
        // Read data from the stream
        match stream.read(&mut buffer).await {
            Ok(0) => {
                // Connection was closed
                eprintln!("Connection closed by the server");
                return Ok(());
            }
            Ok(n) => {
                // Write the data to stdout
                if let Err(e) = tokio::io::stdout().write_all(&buffer[..n]).await {
                    eprintln!("Failed to write to stdout: {}", e);
                    return Err(e);
                }
            }
            Err(e) => {
                // An error occurred
                eprintln!("Failed to read from stream: {}", e);
                return Err(e);
            }
        }
    }
}

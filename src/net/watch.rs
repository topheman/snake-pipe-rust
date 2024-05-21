use crate::net::common::StreamType;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, UnixStream},
    runtime::Runtime,
};

pub fn block_on_watch(props: StreamType) -> std::io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(watch(props))?;
    Ok(())
}

pub async fn watch(props: StreamType) -> std::io::Result<()> {
    match props {
        StreamType::Tcp(bind_addr) => {
            let stream = TcpStream::connect(&bind_addr).await?;
            handle_stream(stream).await
        }
        StreamType::Socket(socket_path) => {
            let stream = UnixStream::connect(socket_path).await?;
            handle_stream(stream).await
        }
    }
}

async fn handle_stream(mut stream: impl AsyncReadExt + std::marker::Unpin) -> std::io::Result<()> {
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

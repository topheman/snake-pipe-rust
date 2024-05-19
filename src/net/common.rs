use std::net::*;

pub enum StreamConstructorArgs {
    Tcp(SocketAddr),
    Socket(std::path::PathBuf),
}

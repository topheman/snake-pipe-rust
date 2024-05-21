pub enum StreamType {
    Tcp(String),
    Socket(std::path::PathBuf),
}

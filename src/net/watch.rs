use crate::net::common::StreamType;
use tokio::runtime::Runtime;

pub fn block_on_watch(props: StreamType) -> std::io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(watch(props));
    Ok(())
}

pub async fn watch(props: StreamType) {}

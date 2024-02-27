use tokio::runtime::Runtime;

use crate::stream_sse::net::bootstrap;

pub fn run(address: String) {
    let rt = Runtime::new().unwrap();
    rt.block_on(bootstrap(address));
}

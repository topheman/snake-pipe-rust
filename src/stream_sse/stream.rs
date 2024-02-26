use crate::stream_sse::net::bootstrap;

pub fn run(address: String) {
    bootstrap(address);
}

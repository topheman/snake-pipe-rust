use crate::render_browser::server::launch_server;

pub fn run(_port: u32, _quiet: bool, _open: bool) {
    println!("Run render_server");
    let _ = launch_server();
}

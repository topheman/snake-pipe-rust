use clap::Subcommand;

#[derive(Subcommand, Copy, Clone)]
pub enum Pipeline {
    /// Play in the terminal
    Play,
    /// Record a party in the terminal
    Record,
    /// Replay a party you recorded in the terminal
    Replay,
    /// Play and share a party via a socket in realtime
    SockPlay,
    /// Render the party you are sharing shrough a socket in realtime
    SockWatch,
    /// Play and share a party through an http server
    HttpPlay,
    /// Render the party you shared through the http server, in the terminal
    HttpWatch,
}

fn format_prefix(prefix: String) -> String {
    if prefix.is_empty() {
        return "".to_string();
    }
    return format!("{:12}", prefix);
}

pub fn generate_command(pipeline: Option<Pipeline>, list: bool, prefix: String) {
    let prefix = format_prefix(prefix);
    match pipeline {
        Some(Pipeline::Play) => println!("{}snakepipe gamestate|snakepipe render", prefix),
        Some(Pipeline::Record) => {
            println!(
                "{}snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render",
                prefix
            )
        }
        Some(Pipeline::Replay) => {
            println!(
                "{}cat /tmp/snakepipe-output|snakepipe throttle|snakepipe render",
                prefix
            )
        }
        Some(Pipeline::SockPlay) => {
            println!(
                "{}snakepipe gamestate|tee /tmp/snakepipe.sock|snakepipe render",
                prefix
            )
        }
        Some(Pipeline::SockWatch) => println!(
            "{}cat /dev/null > /tmp/snakepipe.sock && tail -f /tmp/snakepipe.sock|snakepipe render",
            prefix
        ),
        Some(Pipeline::HttpPlay) => {
            println!(
                "{}snakepipe gamestate|snakepipe render-browser|snakepipe render",
                prefix
            )
        }
        Some(Pipeline::HttpWatch) => println!("{}snakepipe stream-sse|snakepipe render", prefix),
        None => {
            if list {
                println!("List of pipelines\n");
                generate_command(Some(Pipeline::Play), false, "play".to_string());
                generate_command(Some(Pipeline::Record), false, "record".to_string());
                generate_command(Some(Pipeline::Replay), false, "replay".to_string());
                generate_command(Some(Pipeline::SockPlay), false, "sock-play".to_string());
                generate_command(Some(Pipeline::SockWatch), false, "sock-watch".to_string());
                generate_command(Some(Pipeline::HttpPlay), false, "http-play".to_string());
                generate_command(Some(Pipeline::HttpWatch), false, "http-watch".to_string());
                println!("\nCall snakepipe pipeline <COMMAND>|sh");
            }
        }
    }
}

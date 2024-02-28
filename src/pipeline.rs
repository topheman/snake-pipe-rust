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

pub fn generate_command(pipeline: Pipeline) {
    match pipeline {
        Pipeline::Play => println!("snakepipe gamestate|snakepipe render"),
        Pipeline::Record => {
            println!("snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render")
        }
        Pipeline::Replay => {
            println!("cat /tmp/snakepipe-output|snakepipe throttle|snakepipe render")
        }
        Pipeline::SockPlay => {
            println!("snakepipe gamestate|tee /tmp/snakepipe.sock|snakepipe render")
        }
        Pipeline::SockWatch => println!(
            "cat /dev/null > /tmp/snakepipe.sock && tail -f /tmp/snakepipe.sock|snakepipe render"
        ),
        Pipeline::HttpPlay => {
            println!("snakepipe gamestate|snakepipe render-browser|snakepipe render")
        }
        Pipeline::HttpWatch => println!("snakepipe stream-sse|snakepipe render"),
    }
}

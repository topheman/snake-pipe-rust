use clap::Subcommand;
use owo_colors::OwoColorize;

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
    /// Render the party you are sharing through a socket in realtime
    SockWatch,
    /// Play and share a party through an http server
    HttpPlay,
    /// Render the party you shared through the http server, in the terminal
    HttpWatch,
}

fn print_formatted_pipeline(pipeline: &str, prefix: &str) {
    if prefix.is_empty() {
        println!("{}", pipeline);
    } else {
        println!("  {:12}{}", prefix.bold(), pipeline);
    }
}

pub fn generate_command(pipeline: Option<Pipeline>, list: bool, prefix: &str) {
    match pipeline {
        Some(Pipeline::Play) => {
            print_formatted_pipeline("snakepipe gamestate|snakepipe render", prefix);
        }
        Some(Pipeline::Record) => print_formatted_pipeline(
            "snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render",
            prefix,
        ),
        Some(Pipeline::Replay) => print_formatted_pipeline(
            "cat /tmp/snakepipe-output|snakepipe throttle|snakepipe render",
            prefix,
        ),
        Some(Pipeline::SockPlay) => print_formatted_pipeline(
            "snakepipe gamestate|tee /tmp/snakepipe.sock|snakepipe render",
            prefix,
        ),
        Some(Pipeline::SockWatch) => print_formatted_pipeline(
            "cat /dev/null > /tmp/snakepipe.sock && tail -f /tmp/snakepipe.sock|snakepipe render",
            prefix,
        ),
        Some(Pipeline::HttpPlay) => print_formatted_pipeline(
            "snakepipe gamestate|snakepipe render-browser|snakepipe render",
            prefix,
        ),
        Some(Pipeline::HttpWatch) => {
            print_formatted_pipeline("snakepipe stream-sse|snakepipe render", prefix)
        }
        None => {
            if list {
                println!("{}", "List of pipelines:".bold().underline());
                generate_command(Some(Pipeline::Play), false, "play");
                generate_command(Some(Pipeline::Record), false, "record");
                generate_command(Some(Pipeline::Replay), false, "replay");
                generate_command(Some(Pipeline::SockPlay), false, "sock-play");
                generate_command(Some(Pipeline::SockWatch), false, "sock-watch");
                generate_command(Some(Pipeline::HttpPlay), false, "http-play");
                generate_command(Some(Pipeline::HttpWatch), false, "http-watch");
                println!(
                    "\nTo copy a pipeline, run: {}",
                    "snakepipe pipeline <COMMAND>|pbcopy".bold()
                );
            }
        }
    }
}

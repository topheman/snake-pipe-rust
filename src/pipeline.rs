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
    /// Play and share a party via a shared file in realtime
    FilePlay,
    /// Render the party you are sharing through a file in realtime
    FileWatch,
    /// Play and share a party through an http server
    HttpPlay,
    /// Render the party you are sharing through the http server, in the terminal
    HttpWatch,
    /// Play and share a party through a unix socket
    SocketPlay,
    /// Render the party you are sharing through a unix socket in realtime
    SocketWatch,
    /// Play and share a party through tcp
    TcpPlay,
    /// Render the party you are sharing through tcp in realtime
    TcpWatch,
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
        Some(Pipeline::FilePlay) => print_formatted_pipeline(
            "snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render",
            prefix,
        ),
        Some(Pipeline::FileWatch) => print_formatted_pipeline(
            "cat /dev/null > /tmp/snakepipe-output && tail -f /tmp/snakepipe-output|snakepipe render",
            prefix,
        ),
        Some(Pipeline::HttpPlay) => print_formatted_pipeline(
            "snakepipe gamestate|snakepipe render-browser|snakepipe render",
            prefix,
        ),
        Some(Pipeline::HttpWatch) => {
            print_formatted_pipeline("snakepipe stream-sse|snakepipe render", prefix)
        }
        Some(Pipeline::SocketPlay) => print_formatted_pipeline(
            "snakepipe gamestate|snakepipe socket-play|snakepipe render",
            prefix,
        ),
        Some(Pipeline::SocketWatch) => {
            print_formatted_pipeline("snakepipe socket-watch|snakepipe render", prefix)
        }
        Some(Pipeline::TcpPlay) => print_formatted_pipeline(
            "snakepipe gamestate|snakepipe tcp-play|snakepipe render",
            prefix,
        ),
        Some(Pipeline::TcpWatch) => {
            print_formatted_pipeline("snakepipe tcp-watch|snakepipe render", prefix)
        }
        None => {
            if list {
                println!("{}", "List of pipelines:".bold().underline());
                generate_command(Some(Pipeline::Play), false, "play");
                generate_command(Some(Pipeline::Record), false, "record");
                generate_command(Some(Pipeline::Replay), false, "replay");
                generate_command(Some(Pipeline::FilePlay), false, "file-play");
                generate_command(Some(Pipeline::FileWatch), false, "file-watch");
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

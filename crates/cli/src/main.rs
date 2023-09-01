use clap::{Parser, Subcommand};

use common::stream::process_io;
use common::gamestate::hello;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gamestate {
        /// in ms
        #[arg(short, long, default_value_t = 300)]
        throttle: u32
    },
    Render
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gamestate {throttle} => {
            println!("called gamestate with throttle: {throttle:?}");
            hello();
        },
        Commands::Render => {
            println!("called render");
        },
    }
    println!("Hello, world!");
    process_io();
}

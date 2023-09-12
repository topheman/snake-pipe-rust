use clap::{Parser, Subcommand};

use common::gamestate::run as gamestate_run;
use common::stream::{InitOptions, SizeOption};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const DEFAULT_WIDTH: u32 = 25;
const DEFAULT_HEIGHT: u32 = 25;

#[derive(Subcommand)]
enum Commands {
    Gamestate {
        /// in ms
        #[arg(long, default_value_t = 200)]
        frame_duration: u32,
        /// default 25
        #[arg(long)]
        width: Option<u32>,
        /// default 25
        #[arg(long)]
        height: Option<u32>,
        #[arg(long, default_value_t = false)]
        fit_terminal: bool,
    },
    Render,
}

struct CliOptions<'a> {
    frame_duration: &'a u32,
    width: &'a Option<u32>,
    height: &'a Option<u32>,
    fit_terminal: &'a bool,
}

impl Into<InitOptions> for CliOptions<'_> {
    fn into(self) -> InitOptions {
        let size: SizeOption;
        if self.width.is_some() && self.height.is_some() {
            size = SizeOption {
                width: self.width.unwrap_or(DEFAULT_WIDTH),
                height: self.height.unwrap_or(DEFAULT_HEIGHT),
            }
        } else if self.width.is_some() {
            size = SizeOption {
                width: self.width.unwrap_or(DEFAULT_WIDTH),
                height: self.width.unwrap_or(DEFAULT_HEIGHT),
            }
        } else if self.fit_terminal.eq(&true) {
            let (width, height) = crossterm::terminal::size()
                .unwrap_or((DEFAULT_WIDTH as u16, DEFAULT_HEIGHT as u16));
            size = SizeOption {
                width: width as u32,
                height: height as u32,
            }
        } else {
            size = SizeOption {
                width: DEFAULT_WIDTH,
                height: DEFAULT_HEIGHT,
            }
        }
        return InitOptions {
            frame_duration: *self.frame_duration,
            size,
        };
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gamestate {
            frame_duration,
            width,
            height,
            fit_terminal,
        } => {
            let cli_options = CliOptions {
                frame_duration: frame_duration,
                width: width,
                height: height,
                fit_terminal: fit_terminal,
            };
            let game_options: InitOptions = cli_options.into();

            let _ = gamestate_run(game_options);
        }
        Commands::Render => {
            println!("called render");
        }
    }
}

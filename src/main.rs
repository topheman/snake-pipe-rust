use clap::{Parser, Subcommand};
use crossterm;

use snakepipe::common::format_version_to_display;
use snakepipe::gamestate::run as gamestate_run;
use snakepipe::render::run as render_run;
use snakepipe::stream::{InitOptions, SizeOption};
use snakepipe::throttle::run as throttle_run;

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
        #[arg(long, default_value_t = 120)]
        frame_duration: u32,
        /// default 25
        #[arg(long)]
        width: Option<u32>,
        /// default 25
        #[arg(long)]
        height: Option<u32>,
        #[arg(long, default_value_t = 2)]
        snake_length: u32,
        #[arg(long, default_value_t = false)]
        fit_terminal: bool,
    },
    Render,
    Throttle {
        /// in ms
        #[arg(long, default_value_t = 120)]
        frame_duration: u32,
    },
}

struct CliOptions<'a> {
    frame_duration: &'a u32,
    width: &'a Option<u32>,
    height: &'a Option<u32>,
    snake_length: &'a u32,
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
                .unwrap_or((DEFAULT_WIDTH as u16 + 2, DEFAULT_HEIGHT as u16 + 6));
            size = SizeOption {
                width: width as u32 - 2,   // 2 borders
                height: height as u32 - 6, // 2 borders + 4 lines of score/etc ...
            }
        } else {
            size = SizeOption {
                width: DEFAULT_WIDTH,
                height: DEFAULT_HEIGHT,
            }
        }
        let mut version = std::collections::HashMap::new();
        version.insert("gamestate".to_string(), format_version_to_display());
        return InitOptions {
            frame_duration: *self.frame_duration,
            snake_length: *self.snake_length,
            size,
            version: version,
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
            snake_length,
            fit_terminal,
        } => {
            let cli_options = CliOptions {
                frame_duration: frame_duration,
                width: width,
                height: height,
                snake_length: snake_length,
                fit_terminal: fit_terminal,
            };
            let game_options: InitOptions = cli_options.into();

            // enable_raw_mode()?; // https://docs.rs/crossterm/0.27.0/crossterm/terminal/index.html#raw-mode
            let _ = crossterm::terminal::enable_raw_mode();
            let _ = gamestate_run(game_options); // this function returns when ctrl+c is hit
            let _ = crossterm::terminal::disable_raw_mode();
            std::process::exit(130); // todo handle other signals ?
        }
        Commands::Render => {
            render_run();
        }
        Commands::Throttle { frame_duration } => throttle_run(*frame_duration),
    }
}
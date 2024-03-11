use clap::{Parser, Subcommand, ValueEnum};
use crossterm;

use crate::common::format_version_to_display;
use crate::input::{InitOptions, SizeOption};
use crate::pipeline::Pipeline;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

const DEFAULT_WIDTH: u32 = 25;
const DEFAULT_HEIGHT: u32 = 25;

#[derive(Subcommand)]
pub enum Commands {
    /// Accepts user inputs (arrow keys to control the snake) and outputs the state of the game to stdout
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
    /// Reads gamestate from stdin and renders the game on your terminal
    Render,
    /// Reads stdin line by line and outputs each line on stdout each `frame_duration` ms (usefull for replaying a file)
    Throttle {
        /// in ms
        #[arg(long, default_value_t = 120)]
        frame_duration: u32,
        #[arg(long)]
        loop_infinite: bool,
    },
    /// Let's you render the game in your browser at http://localhost:8080 by spawning a server and sending stdin via server-sent events to a JavaScript renderer
    RenderBrowser {
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
    /// Connects to the server spawned by `render-browser` and streams server-sent events back to the terminal
    StreamSse {
        #[arg(long, default_value = "http://localhost:8080")]
        address: String,
    },
    /// Prints out some common pipelines, so that you can copy/paste them to execute (you can pipe to `pbcopy`)
    #[command(arg_required_else_help = true)]
    Pipeline(PipelineArgs),
    /// Generate completions for your own shell (shipped with the homebrew version)
    GenerateCompletions(GenerateCompletionsArgs),
}

#[derive(Parser)]
pub struct PipelineArgs {
    #[command(subcommand)]
    pub sub: Option<Pipeline>,
    #[arg(long)]
    pub list: bool,
}

#[derive(Parser)]
pub struct GenerateCompletionsArgs {
    #[arg(long, value_enum)]
    pub shell: AvailableShells,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AvailableShells {
    Bash,
    Fish,
    Zsh,
}

pub struct CliOptions<'a> {
    pub frame_duration: &'a u32,
    pub width: &'a Option<u32>,
    pub height: &'a Option<u32>,
    pub snake_length: &'a u32,
    pub fit_terminal: &'a bool,
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
                height: self.height.unwrap_or(DEFAULT_HEIGHT),
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
        let mut features_with_version = std::collections::HashMap::new();
        features_with_version.insert("gamestate".to_string(), format_version_to_display());
        let metadatas = std::collections::HashMap::new();
        return InitOptions {
            frame_duration: *self.frame_duration,
            snake_length: *self.snake_length,
            size,
            features_with_version: features_with_version,
            metadatas,
        };
    }
}

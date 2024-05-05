use clap::{Parser, Subcommand, ValueEnum};
use crossterm;

use crate::common::format_version_to_display;
use crate::input::{InitOptions, SizeOption};
use crate::pipeline::Pipeline;

const DEFAULT_UNIX_SOCKET_PATH: &str = "/tmp/snakepipe.sock";
const DEFAULT_TCP_PORT: &str = "8050";
const DEFAULT_TCP_HOST: &str = "127.0.0.1";

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
        /// Specify the length of the snake you want to start with
        snake_length: u32,
        /// Adjust size of the game to the size of your terminal
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
        /// Loop when at the beginning of the stream when it ends
        #[arg(long)]
        loop_infinite: bool,
    },
    /// Renders the game in your browser by spawning a server and sending stdin via server-sent events to a JavaScript renderer
    RenderBrowser {
        /// Override port (default 8080)
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
    /// Connects to the server spawned by `render-browser` and streams server-sent events back to the terminal
    StreamSse {
        /// Override address (default http://localhost:8080)
        #[arg(long, default_value = "http://localhost:8080")]
        address: String,
    },
    /// Accepts gamestate from stdin and pushes it to a unix socket
    SocketPlay {
        /// Unix socket file path
        #[arg(long, default_value = DEFAULT_UNIX_SOCKET_PATH)]
        path: String,
    },
    /// Reads gamestate from a unix socket
    SocketWatch {
        /// Unix socket file path
        #[arg(long, default_value = DEFAULT_UNIX_SOCKET_PATH)]
        path: String,
    },
    /// Accepts gamestate from stdin and pushes it to a tcp socket
    TcpPlay {
        /// Port number
        #[arg(long, default_value = DEFAULT_TCP_PORT)]
        port: u16,
        /// Tcp host
        #[arg(long, default_value = DEFAULT_TCP_HOST)]
        host: String,
    },
    /// Accepts gamestate from stdin and pushes it to a tcp socket
    TcpWatch {
        /// Port number
        #[arg(long, default_value = DEFAULT_TCP_PORT)]
        port: u16,
        /// Tcp host
        #[arg(long, default_value = DEFAULT_TCP_HOST)]
        host: String,
    },
    /// Print some common pipelines to copy/paste and run (you can pipe to `pbcopy`)
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
    /// Specify which shell you target - accepted values: bash, fish, zsh
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

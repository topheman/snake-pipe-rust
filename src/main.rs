use clap::Parser;
use crossterm;

use snakepipe::cli::{Cli, CliOptions, Commands};

use snakepipe::gamestate::run as gamestate_run;
use snakepipe::input::InitOptions;
use snakepipe::pipeline::generate_command as pipeline_generate_command;
use snakepipe::render::run as render_run;
use snakepipe::render_browser::common::port_is_available;
use snakepipe::render_browser::run as render_browser_run;
use snakepipe::stream_sse::run as stream_sse_run;
use snakepipe::throttle::run as throttle_run;

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
        Commands::Throttle {
            frame_duration,
            loop_infinite,
        } => throttle_run(*frame_duration, *loop_infinite),
        Commands::RenderBrowser { port } => {
            if port_is_available(*port) {
                return render_browser_run(*port);
            }
            eprintln!("Error: port {} already in use", port);
            std::process::exit(exitcode::UNAVAILABLE);
        }
        Commands::StreamSse { address } => stream_sse_run(address.to_string()),
        Commands::Pipeline(cmd) => pipeline_generate_command(cmd.sub, cmd.list, ""),
    }
}

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use crossterm;

use snakepipe::cli::{AvailableShells, Cli, CliOptions, Commands};

use snakepipe::gamestate::run as gamestate_run;
use snakepipe::input::InitOptions;
use snakepipe::net::play::{block_on_play, PlayProps};
use snakepipe::pipeline::generate_command as pipeline_generate_command;
use snakepipe::render::run as render_run;
use snakepipe::render_browser::common::port_is_available;
use snakepipe::render_browser::run as render_browser_run;
use snakepipe::stream_sse::run as stream_sse_run;
use snakepipe::throttle::run as throttle_run;
use snakepipe::utils::resolve_path;

fn generate_completion(shell: Shell) {
    generate(
        shell,
        &mut Cli::command(),
        "snakepipe",
        &mut std::io::stdout(),
    )
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
            let _ = gamestate_run(game_options); // this function returns Ok(()) when ctrl+c is hit and Err when it couldn't write to stdout
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
        #[cfg(unix)]
        Commands::SocketPlay { path } => {
            eprintln!("path: {}", path);
            match resolve_path(std::path::PathBuf::from(&path)) {
                Ok(path) => {
                    eprintln!("resolved path: {:?}", &path);
                    match std::fs::remove_file(&path) {
                        Ok(_) => {}
                        Err(err) => {
                            if err.kind() != std::io::ErrorKind::NotFound {
                                eprintln!("err {:?}", err);
                                eprintln!("Failed to remove {:?}", &path);
                                std::process::exit(exitcode::OSFILE);
                            }
                        }
                    }
                    let _ = block_on_play(PlayProps::Socket(path));
                }
                Err(_) => {
                    eprintln!("Could not resolve path {}", path);
                    std::process::exit(exitcode::OSFILE);
                }
            }
        }
        #[cfg(unix)]
        Commands::SocketWatch { path } => {
            eprintln!("path: {}", path);
        }
        Commands::TcpPlay { port, host } => {
            eprintln!("{}:{}", host, port);
            let _ = block_on_play(PlayProps::Tcp(format!("{}:{}", host, port).to_string()));
        }
        Commands::TcpWatch { port, host } => {
            eprintln!("{}:{}", host, port);
        }
        Commands::Pipeline(cmd) => pipeline_generate_command(cmd.sub, cmd.list, ""),
        Commands::GenerateCompletions(flags) => match flags.shell {
            AvailableShells::Bash => generate_completion(Shell::Bash),
            AvailableShells::Fish => generate_completion(Shell::Fish),
            AvailableShells::Zsh => generate_completion(Shell::Zsh),
        },
    }
}

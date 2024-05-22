# snake-pipe-rust

[![crates.io](https://img.shields.io/crates/v/snakepipe.svg)](https://crates.io/crates/snakepipe) [![Docs](https://docs.rs/snakepipe/badge.svg)](https://docs.rs/snakepipe/latest/snakepipe/) [![Build](https://github.com/topheman/snake-pipe-rust/actions/workflows/rust.yml/badge.svg?label=build)](https://github.com/topheman/snake-pipe-rust/actions/workflows/rust.yml)

Not just yet another snake game in the terminal 😉.

https://github.com/topheman/snake-pipe-rust/assets/985982/76161595-1c3a-4252-9cbd-25e144bf185c

This one follows the [unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) as:

- `snakepipe gamestate` accepts user inputs, calculates the state of the game and writes it to `stdout`
- `snakepipe render` reads the state of the game from `stdin` and renders it on the terminal
- `snakepipe throttle` reads a pre-recorded game from `stdin` and writes to `stdout` each tick so that `snakepipe render` can pick it up
- `snakepipe render-browser` spawns a server and sends `stdin` via server-sent events to a JavaScript renderer in your browser
- `snakepipe stream-sse` connects to the server spawned by `render-browser` and streams server-sent events back to the terminal
- `snakepipe socket-play` accepts gamestate from stdin and pushes it to a unix socket
- `snakepipe socket-watch` reads gamestate from a unix socket
- `snakepipe tcp-play` accepts gamestate from stdin and pushes it to a tcp socket
- `snakepipe tcp-watch` reads gamestate from a tcp socket
- `snakepipe pipeline <command>` prints out the most common pipelines (combinations of commands), so that you could directly `pbcopy`/paste them

That way:

- you could write your own version of the `gamestate` or `render` command in any programming language and make it work with mine
- it's a great exercise to handle stream serialization/deserialization in rust

## Motivation

I've already done [a few rust projects](http://labs.topheman.com) (with WebAssembly or [bevy](https://github.com/topheman/bevy-rust-wasm-experiments)), however, I wanted something that needs to deal directly with:

- I/O
- parsing
- parallelism
- async programming
- handling piping/stdin/stdout/signaling ...
- inter-process communication

## Install

Any OS - if you have Rust >= 1.75.0  - [How to install Rust (if you don't have it yet)](https://www.rust-lang.org/tools/install)

```sh
cargo install snakepipe
```

On MacOS, with Homebrew (ships with its own [shell completions](#shell-completions) for zsh, bash and fish)

```sh
brew install topheman/tap/snakepipe
```

Other OS: see [releases](https://github.com/topheman/snake-pipe-rust/releases).

## Usage

### Piping

#### 🎮 Play in terminal

```sh
# basic usage
snakepipe gamestate|snakepipe render

# change the defaults
snakepipe gamestate --frame-duration 80 --width 70 --height 20 --snakepipe-length 15|snakepipe render

# call help on any of the commands
snakepipe --help
```

#### 📼 You can even record and replay using basic piping

```sh
# record a game into a file using the builtin `tee` command utility
snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render

# replay the game you recorded
cat /tmp/snakepipe-output|snakepipe throttle|snakepipe render
```

#### 🖥 You can mirror your playing terminal into a server you can open in a browser

```sh
snakepipe gamestate|snakepipe render-browser|snakepipe render
```

Then open [http://localhost:8080](http://localhost:8080). You'll be able to switch between renderers in your browser as you are playing in your terminal (thanks to server-sent events).

#### 🖼 You can mirror your playing terminal into another one, through http

Open two terminals:

```sh
# main terminal:
# - accepts user inputs
# - spawns an http server that streams stdin to server-sent events
# - renders the game to the terminal so you can play
snakepipe gamestate|snakepipe render-browser|snakepipe render
```

```sh
# mirroring terminal (not necessary the same device, only need to be on the same network):
# - connects to the http server and streams server-sent events to sdout
# - render the gamestate retrieved from the server
snakepipe stream-sse|snakepipe render
```

You could share your game accross your LAN!

### IPC (Inter-process communication)

#### TCP

Open two terminals. `snakepipe tcp-play` will expose a process that accepts tcp connections (on port 8050 by default). You can connect to it via [netcat](https://en.wikipedia.org/wiki/Netcat) (the `nc` command), that will pipe the tcp stream output to stdout.

```sh
# main terminal
snakepipe gamestate|snakepipe tcp-play|snakepipe render
```

```sh
# mirroring terminal
nc localhost 8050|snakepipe render # with netcat
snakepipe tcp-watch|snakepipe render # or with snakepipe itself
```

#### Unix domain sockets

Open two terminals. `snakepipe socket-play` will expose a [unix domain socket](https://en.wikipedia.org/wiki/Unix_domain_socket) (by default on `/tmp/snakepipe.sock`). You can connect to it via [netcat](https://en.wikipedia.org/wiki/Netcat) (the `nc` command), that will pipe the socket stream to stdout.

```sh
# main terminal
snakepipe gamestate|snakepipe socket-play|snakepipe render
```

```sh
# mirroring terminal
nc -U /tmp/snakepipe.sock|snakepipe render # with netcat
snakepipe socket-watch|snakepipe render # or with snakepipe itself
```

### Others

#### 📺 You can also mirror your playing terminal into another one

You should prefer using IPC.

Open two terminals that will communicate via a file that will be `tail`ed and piped to `snakepipe render`

```sh
# mirroring terminal
cat /dev/null > /tmp/snakepipe-output && tail -f /tmp/snakepipe-output|snakepipe render
```

```sh
# main terminal
snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render
```

### 😉 And maybe you'll find other ways?...

## Shell completions

If you install `snakepipe` with Homebrew, it ships with its own completions for zsh, bash and fish and they will be installed without you having to do anything.

If you installed `snakepipe` manually, you can generate the completions files with the `snakepipe generate-completions` command.

## Manual of commands

<details>
  <summary><code>snakepipe --help</code></summary>
  <pre>A snake game based on stdin/stdout following unix philosophy

Usage: snakepipe \<COMMAND>

Commands:

  gamestate       Accepts user inputs (arrow keys to control the snake) and outputs the state of the game to stdout
  render          Reads gamestate from stdin and renders the game on your terminal
  throttle        Reads stdin line by line and outputs each line on stdout each `frame_duration` ms (usefull for replaying a file)
  render-browser  Let's you render the game in your browser at http://localhost:8080 by spawning a server and sending stdin via server-sent events to a JavaScript renderer
  stream-sse      Connects to the server spawned by `render-browser` and streams server-sent events back to the terminal
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
  </pre>
</details>

<details>
  <summary><code>snakepipe gamestate --help</code></summary>
  <pre>Accepts user inputs (arrow keys to control the snake) and outputs the state of the game to stdout

Usage: snakepipe gamestate [OPTIONS]

Options:
      --frame-duration \<FRAME_DURATION>  in ms [default: 120]
      --width \<WIDTH>                    default 25
      --height \<HEIGHT>                  default 25
      --snake-length \<SNAKE_LENGTH>      [default: 2]
      --fit-terminal
  </pre>
</details>

<details>
  <summary><code>snakepipe render --help</code></summary>
  <pre>
Reads gamestate from stdin and renders the game on your terminal

Usage: snakepipe render
  </pre>
</details>

<details>
  <summary><code>snakepipe throttle --help</code></summary>
  <pre>
Reads stdin line by line and outputs each line on stdout each `frame_duration` ms (usefull for replaying a file)

Usage: snakepipe throttle [OPTIONS]

Options:
      --frame-duration \<FRAME_DURATION>  in ms [default: 120]
      --loop-infinite
  </pre>
</details>

<details>
  <summary><code>snakepipe render-browser --help</code></summary>
  <pre>
Let's you render the game in your browser at http://localhost:8080 by spawning a server and sending stdin via server-sent events to a JavaScript renderer

Usage: snakepipe render-browser [OPTIONS]

Options:
      --port <PORT>  [default: 8080]
  </pre>
</details>

<details>
  <summary><code>snakepipe stream-sse --help</code></summary>
  <pre>
Connects to the server spawned by `render-browser` and streams server-sent events back to the terminal

Usage: snakepipe stream-sse [OPTIONS]

Options:
      --address \<ADDRESS>  [default: http://localhost:8080]
  </pre>
</details>

<details>
  <summary><code>snakepipe socket-play --help</code></summary>
  <pre>
Accepts gamestate from stdin and pushes it to a unix socket

Usage: snakepipe socket-play [OPTIONS]

Options:
      --path \<PATH>  Unix socket file path [default: /tmp/snakepipe.sock]
  </pre>
</details>

<details>
  <summary><code>snakepipe socket-watch --help</code></summary>
  <pre>
Reads gamestate from a unix socket

Usage: snakepipe socket-watch [OPTIONS]

Options:
      --path \<PATH>  Unix socket file path [default: /tmp/snakepipe.sock]
  </pre>
</details>

<details>
  <summary><code>snakepipe tcp-play --help</code></summary>
  <pre>
Accepts gamestate from stdin and pushes it to a tcp socket

Usage: snakepipe tcp-play [OPTIONS]

Options:
      --port \<PORT>  Port number [default: 8050]
      --host \<HOST>  Tcp host [default: 127.0.0.1]
  </pre>
</details>

<details>
  <summary><code>snakepipe tcp-watch --help</code></summary>
  <pre>
Reads gamestate from a tcp socket

Usage: snakepipe tcp-watch [OPTIONS]

Options:
      --port \<PORT>  Port number [default: 8050]
      --host \<HOST>  Tcp host [default: 127.0.0.1]
  </pre>
</details>

<details>
  <summary><code>snakepipe pipeline --help</code></summary>
  <pre>
Prints out some common pipelines, so that you can copy/paste them to execute (you can pipe to `pbcopy`)

Usage: snakepipe pipeline [OPTIONS] [COMMAND]

Commands:
  play        Play in the terminal
  record      Record a party in the terminal
  replay      Replay a party you recorded in the terminal
  file-play   Play and share a party via a shared file in realtime
  file-watch  Render the party you are sharing through a file in realtime
  http-play   Play and share a party through an http server
  http-watch  Render the party you shared through the http server, in the terminal
  </pre>
</details>

## Using as a library

```sh
cargo add snakepipe # add it to your project
```

This crate is a cli, but it also exports a lib from where you can import a few utilities, such as `snakepipe::stream::parse_gamestate` - [direct link to docs.rs](https://docs.rs/snakepipe/latest/snakepipe/stream/fn.parse_gamestate.html):

```rust
use snakepipe::stream::{parse_gamestate, Game};

fn main() -> () {
    match parse_gamestate() {
        Ok(stream) => {
            println!(
                "Frame duration {}, Snake length {}, Level {}x{}",
                stream.options.frame_duration,
                stream.options.snake_length,
                stream.options.size.width,
                stream.options.size.height
            );
            for parsed_line in stream.lines {
                do_something(parsed_line);
            }
        }
        Err(e) => {
            println!("Error occurred while parsing stdin: \"{}\"", e);
        }
    }
}

fn do_something(parsed_line: Game) {
    println!("Snake head position {:?}", parsed_line.snake.head)
}
```

## Contributing

You can:

- Make an implementation of the actual `snakepipe render` command for the terminal in an other language than rust
- Make your own JavaScript renderer for the `snakepipe render-browser` command and ask for a PR to integrate it to the project

An Experimental/Partial nodejs implementation of this crate available at [topheman/snake-pipe-node](https://github.com/topheman/snake-pipe-node).

More infos in [CONTRIBUTING.md](./CONTRIBUTING.md).

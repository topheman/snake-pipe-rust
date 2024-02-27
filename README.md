# snake-pipe-rust

[![crates.io](https://img.shields.io/crates/v/snakepipe.svg)](https://crates.io/crates/snakepipe) [![Docs](https://docs.rs/snakepipe/badge.svg)](https://docs.rs/snakepipe/latest/snakepipe/) [![Build](https://github.com/topheman/snake-pipe-rust/actions/workflows/rust.yml/badge.svg?label=build)](https://github.com/topheman/snake-pipe-rust/actions/workflows/rust.yml)

Not just yet another snake game in the terminal 😉.

https://github.com/topheman/snake-pipe-rust/assets/985982/76161595-1c3a-4252-9cbd-25e144bf185c

This one follows the [unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) as:

- `snakepipe gamestate` accepts user inputs, calculates the state of the game and writes it to `stdout`
- `snakepipe render` reads the state of the game from `stdin` and renders it on the terminal
- `snakepipe throttle` reads a pre-recorded game from `stdin` and writes to `stdout` each tick so that `snakepipe render` can pick it up
- `snakepipe render-browser` spawns a server and sends `stdin` via server-sent-events to a JavaScript renderer in your browser
- `snakepipe stream-sse` connects to the server spawned by `render-browser` and streams server-sent-events back to the terminal

That way:

- you could write your own version of the `gamestate` or `render` command in any programming language and make it work with mine, like this one in nodejs: [topheman/snake-pipe-node](https://github.com/topheman/snake-pipe-node)
- it's a great exercise to handle stream serialization/deserialization in rust

## Prerequisites

- Rust 1.75.0 - [How to install Rust (if you don't have it yet)](https://www.rust-lang.org/tools/install)

## Install

```sh
cargo install snakepipe
```

## Usage

### 🎮 Play in terminal

```sh
# basic usage
snakepipe gamestate|snakepipe render

# change the defaults
snakepipe gamestate --frame-duration 80 --width 70 --height 20 --snakepipe-length 15|snakepipe render

# call help on any of the commands
snakepipe --help
```

### 📼 You can even record and replay using basic piping

```sh
# record a game into a file using the builtin `tee` command utility
snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render

# replay the game you recorded
cat /tmp/snakepipe-output|snakepipe throttle|snakepipe render
```

### 📺 You can also mirror your playing terminal into another one

Open two terminals that will communicate via a file that will be `tail`ed and piped to `snakepipe render`

```sh
# mirroring terminal
cat /dev/null > /tmp/snakepipe.sock && tail -f /tmp/snakepipe.sock|snakepipe render
```

```sh
# main terminal
snakepipe gamestate|tee /tmp/snakepipe.sock|snakepipe render
```

### 🖥 You can mirror your playing terminal into a server you can open in a browser

```sh
snakepipe gamestate|snakepipe render-browser|snakepipe render
```

Then open [http://localhost:8080](http://localhost:8080). You'll be able to switch between renderers in your browser as you are playing in your terminal (thanks to server-sent-events).

### 🖼 You can mirror your playing terminal into another one, through http

Open two terminals:

```sh
# main terminal:
# - accepts user inputs
# - spawns an http server that streams stdin to server-sent-events
# - renders the game to the terminal so you can play
snakepipe gamestate|snakepipe render-browser|snakepipe render
```

```sh
# mirroring terminal (not necessary the same device, only need to be on the same network):
# - connects to the http server and streams server-sent-events to sdout
# - render the gamestate retrieved from the server
snakepipe stream-sse|snakepipe render
```

You could share your game accross your LAN!

### 😉 And maybe you'll find other ways?...

## Manual of commands

<details>
  <summary><code>snakepipe --help</code></summary>
  <pre>
Usage: snakepipe <CMD_>

Commands:
  gamestate
  render
  throttle
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
  </pre>
</details>

<details>
  <summary><code>snakepipe gamestate --help</code></summary>
  <pre>
Usage: snakepipe gamestate [OPTIONS]

Options:
      --frame-duration <FRAME_DURATION>  in ms [default: 120]
      --width <WIDTH_>                    default 25
      --height <HEIGHT_>                  default 25
      --snakepipe-length <snakepipe_LENGTH>      [default: 2]
      --fit-terminal
  </pre>
</details>

<details>
  <summary><code>snakepipe render --help</code></summary>
  <pre>
Usage: snakepipe render
  </pre>
</details>

<details>
  <summary><code>snakepipe throttle --help</code></summary>
  <pre>
Usage: snakepipe throttle [OPTIONS]

Options:
      --frame-duration <FRAME_DURATION>  in ms [default: 120]
      --loop-infinite
  </pre>
</details>

<details>
  <summary><code>snakepipe render-browser --help</code></summary>
  <pre>
Usage: snakepipe render-browser [OPTIONS]

Options:
      --port \<PORT>  [default: 8080]
  </pre>
</details>

<details>
  <summary><code>snakepipe stream-sse --help</code></summary>
  <pre>
Connects to the server spawned by `render-browser` and streams server-sent-events back to the terminal

Usage: snakepipe stream-sse [OPTIONS]

Options:
      --address \<ADDRESS>  [default: http://localhost:8080]
  </pre>
</details>

## Using as a library

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

## Next

- [ ] Make an implementation of the `render` command that starts a server so that the render happens in a browser
- [ ] Make an implementation of the actual `render` for the terminal in an other language than rust
- Experimental/Partial nodejs implementation of this crate available at [topheman/snake-pipe-node](https://github.com/topheman/snake-pipe-node)

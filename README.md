# snake-pipe-rust

Not just yet another snake game in the terminal 😉.

https://github.com/topheman/snake-pipe-rust/assets/985982/57b7e5e1-ef0b-4047-8079-f5d455b05a25

This one follows the [unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) as:

- the `gamestate` command accepts user inputs, calculates the state of the game and writes it to `stdout`
- the `render` command reads the state of the game from `stdin` and renders it on the terminal
- the `throttle` command reads a pre-recorded game from `stdin` and writes to `stdout` each tick so that `render` can pick it up

That way:

- you could write your own version of the `gamestate` or `render` command in any programming language and make it work with mine
- it's a great exercise to handle stream serialization/deserialization in rust

## Prerequisites

- Rust 1.75.0

## Usage

Build the project by running: `cargo build`.

🎮 Play in terminal:

- takes user inputs and writes gamestate into stdout
  - `./target/debug/snake gamestate`
- same but faster, with a bigger level and starting with a bigger snake at begining
  - `./target/debug/snake gamestate --frame-duration 80 --width 70 --height 20 --snake-length 15`
- play snake rendered in the terminal
  - `./target/debug/snake gamestate|./target/debug/snake render`

📼 You can even record and replay using basic piping.

- record using the [`tee` command utility](https://en.wikipedia.org/wiki/Tee_(command))
  - `./target/debug/snake gamestate|tee /tmp/snake-output|./target/debug/snake render`
- replay the game recorded previously
  - `cat /tmp/snake-output|./target/debug/snake throttle|./target/debug/snake render`

## Manual of commands

<details>
  <summary><code>./target/debug/snake --help</code></summary>
  <pre>
Usage: snake <CMD_>

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
  <summary><code>./target/debug/snake gamestate --help</code></summary>
  <pre>
Usage: snake gamestate [OPTIONS]

Options:
      --frame-duration <FRAME_DURATION>  in ms [default: 120]
      --width <WIDTH_>                    default 25
      --height <HEIGHT_>                  default 25
      --snake-length <SNAKE_LENGTH>      [default: 2]
      --fit-terminal
  </pre>
</details>

<details>
  <summary><code>./target/debug/snake render --help</code></summary>
  <pre>
Usage: snake render
  </pre>
</details>

<details>
  <summary><code>./target/debug/snake throttle --help</code></summary>
  <pre>
Usage: snake throttle [OPTIONS]

Options:
      --frame-duration <FRAME_DURATION>  in ms [default: 120]
  </pre>
</details>

## Next

- [ ] Make an implementation of the `render` command that starts a server so that the render happens in a browser
- [ ] Make an implementation of the actual `render` for the terminal in an other language than rust

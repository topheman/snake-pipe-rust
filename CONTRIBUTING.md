# Contributing

This is a rust project, but you can contribute as a rust developer as much as a JavaScript developer.

## Rust

You will find your way in the the source code: [`./src`](./src/).

To build the project:

```sh
cargo build
```

To run:

```sh
./target/debug/snakepipe # will show the help - you can use it is explained in the README
```

## JavaScript

The [`snakepipe render-browser`](./README.md#-you-can-mirror-your-playing-terminal-into-another-one-through-http) command launches a rust http server that serves some JavaScript code that connects to server-sent events and renders the game inside the browser.

The source code for the renderers is available at [`static/renderers`](static/renderers).

**You don't need rust to work on this part**. I made a nodejs development server that acts as the `snakepipe render-browser` command (that way, you also don't have to re-build the rust part each time you modify the frontend).

### Install

```sh
npm install
```

### Setup

Build the packages and retrieve a recorded party.

```sh
npm run build && npm run setup
```

### Run

```sh
npm run dev-server-sse
```

Go to [http://localhost:8080](http://localhost:8080).

## Shell completions

The `snakepipe` cli comes with a `generate-completions` command that will generate a completions script for either `bash`, `zsh` or `fish`.

To test the completions in local, you need `zsh` (I didn't provide a mechanism for the other shells - it works the same way), all you need to do is:

```sh
cargo build && source ./scripts/test-zsh-completions
```

The script will temporary add the completions for `snakepipe` to the current terminal tab - don't worry, none of your config file are modified, any modification will disappear once you close the current tab.

# Resources

- 📺 [Understand zsh completions](https://www.youtube.com/watch?v=BHxaUP0kz9w)

# Contributing

This is a rust project, but you can contribute as a rust developer as much as a JavaScript developer.

## Rust

You will find your way in the the source code: [`./src`](./src/).

To build the project:

```sh
cargo build
```

## JavaScript

The [`snakepipe render-browser`](./README.md#-you-can-mirror-your-playing-terminal-into-another-one-through-http) command launches a rust http server that serves some JavaScript code that connects to server-sent-events and renders the game inside the browser.

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

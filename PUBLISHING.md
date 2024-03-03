# Publishing

This document is reserved for the publishing part - you don't need it as a consumer or as a contributer, this is for me.

I share it because, some people might find it useful and other might find ways to improve it.

## Setup

### Just

You'll need the task runner [just](https://github.com/casey/just) - see the [install section](https://github.com/casey/just?tab=readme-ov-file#packages).

If you are on MacOS, you can install it with Homebrew:

```sh
brew install just
```

### Compilation targets

Whether you compiling from intel or arm, you will need to add the missing target - see the installed targets: `rustup target list|grep installed`.

```sh
rustup target install x86_64-apple-darwin # if you're on arm and missing the intel target
rustup target install aarch64-apple-darwin # if you're on intel and missing the arm target
```

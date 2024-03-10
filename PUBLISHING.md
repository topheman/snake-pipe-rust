# Publishing

This document is reserved for the publishing part - you don't need it as a consumer or as a contributer, this is for me.

I share it because, some people might find it useful and others might find ways to improve it.

## Homebrew

The homebrew formula for snakepipe is hosted at [topheman/homebrew-tap](https://github.com/topheman/homebrew-tap/blob/main/Formula/snakepipe.rb).

The binaries are available at [topheman/snake-pipe-rust/releases](https://github.com/topheman/snake-pipe-rust/releases).

### Before setup

Add `export HOMEBREW_EDITOR="code -w"` to your `.bashrc`/`.zshrc` if you prefer editing the Formulaes with `vscode`.

### Setup

You only have to do it once, providing a url of a compressed binary:

```sh
brew create --tap topheman/tap https://github.com/topheman/snake-pipe-rust/releases/download/v2.0.0/snakepipe-x86_64-apple-darwin.tar.gz
```

You will prompt with a formula, that you can customize. I added the targets for x86_64 and aarch64.

Close the the editor.

To test the formula in local:

```sh
brew audit --strict --new --online snakepipe
```

Then, you can access the repository that was created for you and customize it / push it to a remote (in my case to [topheman/homebrew-tap](https://github.com/topheman/homebrew-tap)):

```sh
cd $(brew --repository topheman/tap)
```

Sources:

- https://publishing-project.rivendellweb.net/creating-and-running-your-own-homebrew-tap/
- https://github.com/kcctl/homebrew-tap/blob/main/Formula/kcctl.rb

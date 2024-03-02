# Recording notes

Those are the recording notes I use when making the little demo video.

---

Change the `$PS1`

```sh
export PS1='%{%B%}topheman/snake-pipe-rust%{$reset_color%}%  > '
```

---

link `snakepipe` to the debug binary in the project

```sh
SNAKE_PATH="$PWD/target/debug"
export PATH="$SNAKE_PATH:$PATH"
```

---

cmd+K

---

```sh
cargo install snakepipe
```

---

```sh
snakepipe
```

---

```sh
# Run the gamestate command
# which accepts user inputs and passes the game state to stdout

snakepipe gamestate
```

---

```sh
# Pipe the output of gamestate command to the render command

snakepipe gamestate|snakepipe render
```

---

```sh
# Record a party by saving the output of the gamestate command to a file with the built-in tee utility

snakepipe gamestate|tee /tmp/snake-output|snakepipe render
```

---

```sh
# Replay a party by reading the previous file and streaming it to the render command with the throttle command

cat /tmp/snake-output|snakepipe throttle|snakepipe render
```

---

```sh
# Thank you
```

---

Convert `.mov` to `.mp4`

```sh
ffmpeg -i snake-pipe-rust.mov -vcodec h264 -acodec aac snake-pipe-rust.mp4
```

# Recording notes

Those are the recording notes I use when making the little demo video.

---

Change the `$PS1`

```sh
export PS1='%{%B%}topheman/snake-pipe-rust%{$reset_color%}%  > '
```

---

cmd+K

---

```sh
cargo build
```

---

```sh
# Run the gamestate command
# which accepts user inputs and passes the game state to stdout

./target/debug/snake gamestate
```

---

```sh
# Pipe the output of gamestate command to the render command

./target/debug/snake gamestate|./target/debug/snake render
```

---

```sh
# Record a party by saving the output of gamestate command to a file with the built-in tee utility

./target/debug/snake gamestate|tee /tmp/snake-output|./target/debug/snake render
```

---

```sh
# Replay a party by reading the previous file and streaming it to the render command with the throttle command

cat /tmp/snake-output|./target/debug/snake throttle|./target/debug/snake render
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

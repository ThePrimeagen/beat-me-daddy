## USE NIGHTLY
Once you have rust installed, install nightly
`rustup toolchain install nightly`

### look_at_me_daddy
To run the TUI client there are two steps

1.  copy the .env-example to .env and fill in the proper values.
2.  run `cargo run --release --bin look_at_me_daddy`
    You can also run it with `cargo +nightly run --release --bin look_at_me_daddy` for a quick shortcut using nightly.  SMILEY FACE

#### How to use look_at_me_daddy
* use hjkl to move one note at a time
* use w / b to move a quarter note
* use W / B to move a whole note
* space to toggle if the note should be played
* enter to send to twitch

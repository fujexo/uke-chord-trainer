# Chord Trainer for Ukulele

This project was written in Rust for Webassembly, just because I was curious!

I was previously using the Akkord Trainer from ukuleleinsider.de (is offline since). I felt to write something very similar but with added features like an image of the current chord displayed.

The current version is deployed on Github Pages at https://fujexo.github.io/uke-chord-trainer/

## Development

### Requirements

* Rust >= 1.55, see https://github.com/rust-lang/rust/issues/75243
* `wasm32-unknown-unknown` target
* `trunk` 
* `wasm-bindgen-cli`

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

### Build / Run

Simple! just execute `trunk serve` in the repository and the tool will start a webserver which also checks for file changes. How convenient.

If you need a release build, execute `trunk build --release`. If you are deploying at a different path than /, add `--public-url /path/` to the build process.


## Sources

* Metronome Sound: https://bigsoundbank.com/detail-0468-metronome-a-120bpm.html
* Idea: ukuleleinsider.de/akkord-trainer

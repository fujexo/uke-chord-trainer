# Chord Trainer for Ukulele

This project was written in Rust for Webassembly, just because I was curious!

I was previously using the Akkord Trainer from [ukuleleinsider.de](https://ukuleleinsider.de/akkord-trainer/). I felt to write something very similar but with added features like an image of the current chord displayed.

## Development

### Requirements

* Rust >= 1.55, see https://github.com/rust-lang/rust/issues/75243
* `wasm32-unknown-unknown` target
* `trunk` 
* `wasm-bindgen-cli`

Currently, rustc 1.54.0 is shipped as stable (as of 2021-09-07). rustc 1.55 can be installed from the beta channel.

```bash
rustup toolchain install beta
rustup override set beta
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

### Build / Run

Simple! just execute `trunk serve` in the repository and the tool will start a webserver which also checks for file changes. How convenient.

If you need a release build, execute `trunk build --release`.

## TODO

* Make UI more user friendly
    * Contrast!
    * Maybe replace the CSS framework, this one is just from testing
    * Add a button to "Zoom in" the Chords and Chord image after selecting the chords?
* Create svg for all missing chords (based on img/template.svg)
* Check if I really really can't send the play trigger from rust and have to use the js binding

## Sources

* Metronome Sound: https://bigsoundbank.com/detail-0468-metronome-a-120bpm.html
* Idea: https://ukuleleinsider.de/akkord-trainer
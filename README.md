# Minefield

Another implementation of classic minesweeper. Written in Rust with SDL2.

## Build

For all platforms you have to [install Rust](https://www.rust-lang.org/tools/install).

### Unix-like system

Install required development packages from your package manager.
- `libsdl2-dev`
- `libsdl2-image-dev`
- `libsdl2-ttf-dev`

Open terminal. `cd` into directory with source code and run
```
cargo build --release
```
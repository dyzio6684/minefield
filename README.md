# Minefield

Simple implementation of classic minesweeper. Written in Rust with SDL2.

## Gameplay

![gameplay](https://user-images.githubusercontent.com/51453773/179395884-8269248e-2341-46d3-b1f6-6c60dbb4d78d.gif)

Left-click cell to reveal it. Right-click cell to add/remove flag. Press <kbd>N</kbd> to start a new game.

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

## License

This project is licensed under the [MIT License](LICENSE).

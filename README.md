# Chip8-Emu
A straight-to-the-point chip8 emulator built in Rust, I know, quite the original project.
The video, audio and input is provided by SDL2 (via [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2)).

## Usage
```
./chip8-emulator.exe <rom_path>
```
An SDL2 window should open with the game, by default, the keys 0-F are mapped to 1-V on a regular keyboard.

## Building
1. `git clone https://github.com/Linkster78/Chip8-Emu`
2. `cd Chip8-Emu`
3. Install SDL2 as described in [this repository](https://github.com/Rust-SDL2/rust-sdl2).
4. `cargo build --release`
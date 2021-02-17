# Arduino Rust template
This is a template for developing for Arduino Uno using Rust programming language.
In includes:
- Target specification for atmega328p (`./avr-atmega328p.json`)
- Cargo config for cross-compiling to said target (`.cargo/config.toml`)
- A very simple script to enable `cargo run` on linux
- Minimal Rust project containing blink
## Running
Firstly, install all the typical arduino toolchain (`avr-gcc`, `avrdude`, etc.)
Secondly, set up a nightly toolchain and set it as default for the project, since there are some unstable features used.
```sh
rustup toolchain install nightly-2021-01-07
rustup override set nightly-2021-01-07
```
Now you can use `cargo build` to compile this project (resulting binary will be `target/avr-atmega328p/debug/try-arduino.elf`) or `cargo run` to compile and flash this project.
## Flashing script
Currently the script is very limited:
- It only works on platforms capable of running shell scripts and requires `sudo`, since it uses super-user privileges to ensure `avrdude` has access to tty ports
# wasm-audio-example

> WIP

ensure you have Rust installed:

https://www.rust-lang.org/tools/install

After installing Rust, you should be able to run the following commands to install the necessary tools:

1. `rustup add target wasm32-wasi`
2. `cargo install cargo-component`

Then try run the following commands in the root of the project:

1. `./build.sh`
2. `cargo run` this will run the basic example
3. `cargo run -p host-egui` this will run the egui example

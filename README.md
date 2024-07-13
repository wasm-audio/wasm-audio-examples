# wasm-audio-example



https://github.com/user-attachments/assets/10eb64c9-e256-4021-a278-57cb981f3e92



> WIP

ensure you have Rust installed:

https://www.rust-lang.org/tools/install

After installing Rust, you should be able to run the following commands to install the necessary tools:

1. `cargo install cargo-component`

Then try run the following commands in the root of the project:

1. `./build.sh`
2. `cargo run` this will run the basic example
3. `cargo run -p host-egui` this will run the egui example, you can drop the wasm files in `wasm-audio-plugin` folder into the window

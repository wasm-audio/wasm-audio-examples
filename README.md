# wasm-audio-example

> Note, this is just a WIP! 🛠️

## What is it?
The WASM Component allows developers to write audio plugins in various languages (demo provided in Rust and JavaScript), compile them to WebAssembly (WASM), and run them across different platforms. The demo showcases a desktop application built with Rust using egui as the host, which supports drag-and-drop functionality for already compiled .wasm files as guest plugins.

https://github.com/user-attachments/assets/10eb64c9-e256-4021-a278-57cb981f3e92

## Why?

The WebAssembly Component Model (https://component-model.bytecodealliance.org/) represents a new architecture for program development. WASM significantly enhances computation efficiency in browsers and is supported outside browsers by runtimes like Wasmtime. However, WASM lacks a unified interface, which the WebAssembly Component Model addresses. This model enables the creation of a simple API focused on audio logic, which can be directly compiled to WASM.

For example, in the guest-mul scale module, the developer only needs to write a few lines of code:

https://github.com/wasm-audio/wasm-audio-examples/blob/main/guest-mul/src/lib.rs

## How to use?

ensure you have Rust installed:

https://www.rust-lang.org/tools/install

After installing Rust, you should be able to run the following commands to install the necessary tools:

1. `cargo install cargo-component`

Then try run the following commands in the root of the project:

1. `./build.sh`
2. `cargo run` this will run the basic example
3. `cargo run -p host-egui` this will run the egui example, you can drop the wasm files in `wasm-audio-plugin` folder into the window

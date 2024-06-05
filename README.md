ensure you have installed:

Then:

1. `rustup add target wasm32-wasi`
2. `cargo install cargo-component`

Then try:

1. `./build.sh`
2. `cargo run` this will run the basic example
3. `./run-host-mod.sh` this will show how to dynamically load an audio wasm module

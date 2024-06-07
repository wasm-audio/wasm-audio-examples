cargo component build -r -p guest-mul

cp target/wasm32-wasi/release/guest_mul.wasm ./wasm-audio-plugin

cargo component build -r -p guest-sin

cp target/wasm32-wasi/release/guest_sin.wasm ./wasm-audio-plugin

cargo component build -r -p guest-techno

cp target/wasm32-wasi/release/guest_techno.wasm ./wasm-audio-plugin

# jco componentize ./guest-mul/mul.js --wit ./guest-mul/wit/world.wit -o ./wasm-audio-plugin/mul-js.wasm

# jco componentize ./guest-sin/sin.js --wit ./guest-sin/wit/world.wit -o ./wasm-audio-plugin/sin-js.wasm


cd wasm-audio-plugin

# jco opt sin-js.wasm -o sin-js-opt.wasm
# jco opt mul-js.wasm -o mul-js-opt.wasm
jco opt guest_mul.wasm -o mul-opt.wasm
jco opt guest_sin.wasm -o sin-opt.wasm
jco opt guest_techno.wasm -o techno-opt.wasm
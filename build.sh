cd guest-mul && cargo component build -r

cd ..

cp target/wasm32-wasi/release/mul.wasm ./wasm-audio-plugin

cd guest-sin && cargo component build -r

cd ..

cp target/wasm32-wasi/release/sin.wasm ./wasm-audio-plugin

cd guest-techno && cargo component build -r

cd ..

cp target/wasm32-wasi/release/techno.wasm ./wasm-audio-plugin

# jco componentize ./guest-mul/mul.js --wit ./guest-mul/wit/world.wit -o ./wasm-audio-plugin/mul-js.wasm

# jco componentize ./guest-sin/sin.js --wit ./guest-sin/wit/world.wit -o ./wasm-audio-plugin/sin-js.wasm


cd wasm-audio-plugin

# jco opt sin-js.wasm -o sin-js-opt.wasm
# jco opt mul-js.wasm -o mul-js-opt.wasm
jco opt mul.wasm -o mul-opt.wasm
jco opt sin.wasm -o sin-opt.wasm
jco opt techno.wasm -o techno-opt.wasm
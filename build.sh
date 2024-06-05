cd guest-mul && cargo component build -r

cd ..

cp target/wasm32-wasi/release/mul.wasm ./

cd guest-sin && cargo component build -r

cd ..

cp target/wasm32-wasi/release/sin.wasm ./

cd guest-techno && cargo component build -r

cd ..

cp target/wasm32-wasi/release/techno.wasm ./


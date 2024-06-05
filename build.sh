cd guest-mul && cargo component build -r

cp target/wasm32-wasi/release/mul.wasm ../

cd ..

cd guest-sin && cargo component build -r

# copy guest/target/wasm32-wasi/release/sineosc.wasm to ./
cp target/wasm32-wasi/release/sin.wasm ../

cd ..
cd component-a
cargo component build --target wasm32-wasip2 --release

cd ..

cd component-b
cargo component build --target wasm32-wasip2 --release

cd ..

wac plug component-a/target/wasm32-wasip2/release/component_a.wasm --plug component-b/target/wasm32-wasip2/release/component_b.wasm -o plugged.wasm

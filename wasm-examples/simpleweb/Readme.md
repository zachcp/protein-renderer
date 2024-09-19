# Simpleweb

Protein Rendering compiled to WASM.


```sh

# rustup target add wasm32-unknown-unknown
# cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web target/wasm32-unknown-unknown/release/simpleweb.wasm

# wasm-opt -Os add_bg.wasm -o add.wasm
wasm-opt -Os pkg/bevy_wasm_test_bg.wasm -o add.wasm
mv add.wasm pkg/bevy_wasm_test_bg.wasm
python3 -m http.server

```

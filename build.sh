cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/tide_wasm.wasm --out-dir ./out --target web

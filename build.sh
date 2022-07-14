cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wgse-bevy --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/wgse-bevy.wasm
wasm-opt -O -ol 100 -s 100 -o ./wasm/target/wgse-bevy_bg.wasm ./wasm/target/wgse-bevy_bg.wasm
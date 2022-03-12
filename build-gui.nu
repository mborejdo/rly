cargo build --target wasm32-unknown-unknown -p rly-gui; 
wasm-bindgen --target web --no-typescript --out-dir "./gui/static/" --out-name wasm "target/wasm32-unknown-unknown/debug/rly-gui.wasm"
grass assets/style.scss > gui/static/style.css
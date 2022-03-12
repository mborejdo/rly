cargo build --release -p rly-cli;
cargo build --release -p rly-server;
cargo build --release --target wasm32-unknown-unknown -p rly-gui;
wasm-bindgen --debug --target web --no-typescript --out-dir "./gui/static/" --out-name wasm "target/wasm32-unknown-unknown/release/rly-gui.wasm"
grass assets/style.scss > gui/static/style.css
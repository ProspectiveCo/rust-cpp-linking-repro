# Repro for C++/Rust linking pain

* Setup a Python 3.12 environment.
* Checkout [emsdk](https://github.com/emscripten-core/emsdk.git) as `.emsdk` in the root.
* `cd .emsdk && ./emsdk install 3.1.58`
* `pip install --upgrade maturin`
* Run `make build_rust_wheel WASM=1`
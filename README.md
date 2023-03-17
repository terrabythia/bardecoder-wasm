# WASM qr scanner

> Note: this is still a beta version of this package so the API may be changing completely until `1.0.0` is out.

Scan QR codes in the browser using WebAssembly and is 100% written in Rust. Uses the awesome Rust library [bardecoder](https://github.com/piderman314/bardecoder) for decoding images with QR codes to their string value. The image data should be provided as base64 data string. In tests it now took about ~80ms for each scan. That includes drawing the current video frame on a canvas, getting the canvas content as a base64 encoded string, sending the data string to wasm and getting a result back. Would like to test this on lower end devices and against other solutions later.

## Install
run `cargo update`

## Build
run `wasm-pack build --release --target web`

## Run example
Run the build script first, then run a webserver in the root of this project. For example using npm: run `npx serve` and navigate to `http://localhost:3000/example`. Hold up some QR

## Running tests
run `cargo test` and `wasm-pack test --node`

Roadmap
- [x] add unit tests for basic functions
- [x] add tests for wasm_bindgen functions
- [x] update example html page to use camera
- [ ] optimize crate feature flags where possible for smaller wasm size (now ~1mb, which is too large in some cases)
- [ ] add benchmarks for slower devices and agains other libraries
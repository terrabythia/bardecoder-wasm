Note: this is still a beta version of this package so the API may be changing completely until `1.0.0` is out. It's working, but missing tests, some benchmarks... `v1.0.0` will come soon!

Scan barcodes and QR codes in the browser using WebAssembly. Uses the Rust library [bardecoder](https://github.com/piderman314/bardecoder) for decoding images with QR codes to their string value. The image data should be provided as base64 data string.

Roadmap
- [x] add unit tests for basic functions
- [x] add tests for wasm_bindgen functions
- [ ] update example html page to use camera
- [ ] optimze crate feature flags where possible for smaller wasm size
//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use bardecoder_wasm::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn my_async_test() {
    // Create a promise that is ready on the next tick of the micro task queue.
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Convert that promise into a future and make the test wait on it.
    let x = JsFuture::from(promise).await.unwrap();
    assert_eq!(x, 42);
}

#[wasm_bindgen_test]
async fn async_decoder_test() {
    let base_64_data = include_str!("./fixtures/png_base64_image.txt");
    let base_64_pattern = "data:image/png;base64,";
    let result: Result<JsValue, JsValue> =
        decode_base64(&format!("{}{}", base_64_pattern, base_64_data)).await;

    let unwrapped_result: Vec<String> = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();

    assert_eq!(unwrapped_result.len(), 1);
    assert_eq!(unwrapped_result[0], "http://my.qrvoice.net/1moVKIJ");
}

#[wasm_bindgen_test]
async fn async_decoder_png_test() {
    let result: Result<JsValue, JsValue> =
        decode_base64_png(include_str!("./fixtures/png_base64_image.txt")).await;

    let unwrapped_result: Vec<String> = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();

    assert_eq!(unwrapped_result.len(), 1);
    assert_eq!(unwrapped_result[0], "http://my.qrvoice.net/1moVKIJ");
}

#[wasm_bindgen_test]
async fn async_decoder_jpeg_test() {
    let result: Result<JsValue, JsValue> =
        decode_base64_jpeg(include_str!("./fixtures/jpeg_base64_image.txt")).await;

    let unwrapped_result: Vec<String> = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();

    assert_eq!(unwrapped_result.len(), 1);
    assert_eq!(unwrapped_result[0], "http://my.qrvoice.net/1moVKIJ");
}

#[wasm_bindgen_test]
async fn async_decoder_gif_test() {
    let result: Result<JsValue, JsValue> =
        decode_base64_gif(include_str!("./fixtures/gif_base64_image.txt")).await;

    let unwrapped_result: Vec<String> = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();

    assert_eq!(unwrapped_result.len(), 1);
    assert_eq!(unwrapped_result[0], "http://my.qrvoice.net/1moVKIJ");
}

#[wasm_bindgen_test]
async fn async_decoder_webp_test() {
    let result: Result<JsValue, JsValue> =
        decode_base64_webp(include_str!("./fixtures/webp_base64_image.txt")).await;

    let unwrapped_result: Vec<String> = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();

    assert_eq!(unwrapped_result.len(), 1);
    assert_eq!(unwrapped_result[0], "http://my.qrvoice.net/1moVKIJ");
}

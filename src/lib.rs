use base64::{engine::general_purpose, Engine as _};
use image;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type QrResult = Result<Vec<String>, String>;

fn qr_from_base64_string(base64: &str, format: image::ImageFormat) -> QrResult {
    let bytes = match general_purpose::STANDARD.decode(base64) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(format!("Error decoding base64 data: {}", e));
        }
    };

    let img = match image::load_from_memory_with_format(&bytes, format) {
        Ok(img) => img,
        Err(e) => {
            return Err(format!("Error loading image from memory: {}", e));
        }
    };

    let decoder = bardecoder::default_decoder();
    let results = decoder.decode(&img);

    let valid_results = results
        .iter()
        .filter_map(|result| result.as_ref().ok())
        .cloned()
        .collect::<Vec<String>>();

    Ok(valid_results)
}

fn wrap_qr_result(result: QrResult) -> Result<JsValue, JsValue> {
    match result {
        Ok(results) => serde_wasm_bindgen::to_value(&results).map_err(|e| {
            println!("Error converting results to JsValue: {}", e);
            JsValue::from_str("Error converting results to JsValue")
        }),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

#[wasm_bindgen]
pub async fn decode_base64(base64: &str) -> Result<JsValue, JsValue> {
    let (mime, pattern) = match base64.find(',') {
        Some(index) => {
            let mime = &base64[5..index];
            let pattern = &base64[..index + 1];
            (mime, pattern)
        }
        None => return Err(JsValue::from_str("Invalid base64 data")),
    };

    match mime {
        "jpg" | "jpeg" => decode_base64_jpeg(&base64[pattern.len()..]).await,
        "png" => decode_base64_png(&base64[pattern.len()..]).await,
        "gif" => decode_base64_gif(&base64[pattern.len()..]).await,
        "webp" => decode_base64_webp(&base64[pattern.len()..]).await,
        _ => Err(JsValue::from_str("Invalid base64 data")),
    }
}

#[wasm_bindgen]
pub async fn decode_base64_png(base64: &str) -> Result<JsValue, JsValue> {
    wrap_qr_result(qr_from_base64_string(base64, image::ImageFormat::Png))
}

#[wasm_bindgen]
pub async fn decode_base64_jpeg(base64: &str) -> Result<JsValue, JsValue> {
    wrap_qr_result(qr_from_base64_string(base64, image::ImageFormat::Jpeg))
}

#[wasm_bindgen]
pub async fn decode_base64_gif(base64: &str) -> Result<JsValue, JsValue> {
    wrap_qr_result(qr_from_base64_string(base64, image::ImageFormat::Gif))
}

#[wasm_bindgen]
pub async fn decode_base64_webp(base64: &str) -> Result<JsValue, JsValue> {
    wrap_qr_result(qr_from_base64_string(base64, image::ImageFormat::WebP))
}

// add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_from_base64_string() {
        let test_formats = [
            (
                image::ImageFormat::Png,
                include_str!("../tests/fixtures/png_base64_image.txt"),
            ),
            (
                image::ImageFormat::Jpeg,
                include_str!("../tests/fixtures/jpeg_base64_image.txt"),
            ),
            (
                image::ImageFormat::Gif,
                include_str!("../tests/fixtures/gif_base64_image.txt"),
            ),
            (
                image::ImageFormat::WebP,
                include_str!("../tests/fixtures/webp_base64_image.txt"),
            ),
        ];

        for (format, base64_string) in test_formats.iter() {
            let result = qr_from_base64_string(base64_string, *format);
            assert!(result.is_ok());
            let results = result.unwrap();
            assert_eq!(results.len(), 1);
            assert_eq!(results[0], "http://my.qrvoice.net/1moVKIJ");
        }
    }
}

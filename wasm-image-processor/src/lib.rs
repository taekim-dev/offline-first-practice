use wasm_bindgen::prelude::*;
use image::{ImageBuffer, Rgba};
use base64::{Engine as _, engine::general_purpose};
use js_sys::{Uint8Array, Uint8ClampedArray};
use web_sys::{ImageData, CanvasRenderingContext2d};
use console_error_panic_hook;

// Debug macro for logging to browser console
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// This is our secret algorithm parameters
const GRAYSCALE_WEIGHTS: [f32; 3] = [0.2989, 0.5870, 0.1140]; // Standard RGB to grayscale weights
const PROCESSING_KEY: [u8; 32] = [
    // This would be your secret key for image processing
    0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
    0x13, 0x24, 0x35, 0x46, 0x57, 0x68, 0x79, 0x8a,
];

#[wasm_bindgen]
pub fn process_image(data: Uint8ClampedArray, width: u32, height: u32) -> Result<ImageData, JsValue> {
    console_log!("Processing image: {}x{}", width, height);

    // Convert Uint8ClampedArray to Vec<u8>
    let mut pixels: Vec<u8> = vec![0; (width * height * 4) as usize];
    data.copy_to(&mut pixels[..]);

    console_log!("Converted to pixel array, length: {}", pixels.len());

    // Create image buffer
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, pixels)
        .ok_or_else(|| JsValue::from_str("Failed to create image buffer"))?;

    console_log!("Created image buffer");

    // Apply our "secret" processing algorithm
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Get RGB values
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        // Calculate weighted grayscale value with our secret weights
        let gray_value = (
            r * GRAYSCALE_WEIGHTS[0] +
            g * GRAYSCALE_WEIGHTS[1] +
            b * GRAYSCALE_WEIGHTS[2]
        ) as u8;

        // Apply additional processing using our secret key
        let processed_value = apply_secret_processing(gray_value);

        if x == 0 && y == 0 {
            console_log!(
                "First pixel: R:{} G:{} B:{} -> Gray:{} -> Processed:{}",
                r, g, b, gray_value, processed_value
            );
        }

        // Set RGB channels to the processed value
        pixel[0] = processed_value;
        pixel[1] = processed_value;
        pixel[2] = processed_value;
        // Keep alpha channel unchanged
    }

    console_log!("Finished processing pixels");

    // Convert back to ImageData
    let processed_pixels = img.into_raw();
    let uint8_array = Uint8ClampedArray::new_with_length(processed_pixels.len() as u32);
    uint8_array.copy_from(&processed_pixels[..]);

    console_log!("Converting back to ImageData");

    ImageData::new_with_u8_clamped_array_and_sh(uint8_array.as_ref(), width, height)
}

// Our "secret" processing function
#[inline(never)] // Make it harder to inline and reverse engineer
fn apply_secret_processing(value: u8) -> u8 {
    let mut result = value;
    
    // Apply some "secret" transformations using our key
    for (i, &key_byte) in PROCESSING_KEY.iter().enumerate() {
        result = result.wrapping_add(key_byte);
        if i % 2 == 0 {
            result = result.rotate_left(1);
        } else {
            result = result.rotate_right(1);
        }
    }

    // Additional processing that's hard to reverse
    result = result ^ (result >> 4);
    result = result.wrapping_mul(0x9E);
    result = result ^ (result << 2);

    result
}

// Helper function to validate access token before processing
#[wasm_bindgen]
pub fn validate_processing_token(token: &str) -> bool {
    console_log!("Validating token: {}", token);
    // In a real implementation, this would validate the token
    // using the same HMAC logic as your auth system
    true // For now, always return true
} 
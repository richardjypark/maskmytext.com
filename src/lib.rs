mod utils;
mod text_processor;
mod case_utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Set;

pub use case_utils::{determine_case_suffix, capitalize_first};

/// A WebAssembly module for text masking and obfuscation.
/// 
/// This library provides functions for masking sensitive words in text with
/// asterisks or field placeholders, and for decoding obfuscated text.

/// Returns a greeting message, primarily used to test that the WASM module loaded properly.
/// 
/// # Returns
/// 
/// A String containing the greeting message.
#[wasm_bindgen]
pub fn greet() -> String {
    let message = "Hello, console log message mask-my-text from Rust!";
    // Log for visual feedback
    console::log_1(&JsValue::from_str(message));
    message.to_string()
}

/// Masks specified words in text with asterisks.
/// 
/// Replaces each occurrence of words from the provided set with asterisks
/// matching the length of the original word. Preserves case sensitivity.
/// 
/// # Parameters
/// 
/// * `text` - The original text to mask
/// * `mask_words` - A JavaScript Set containing the words to mask
/// 
/// # Returns
/// 
/// A String with the specified words masked with asterisks.
#[wasm_bindgen]
pub fn mask_text(text: String, mask_words: &Set) -> String {
    text_processor::mask_text(text, mask_words)
}

/// Masks specified words in text with numbered field placeholders.
/// 
/// Replaces each occurrence of words from the provided set with field placeholders
/// in the format "FIELD_N". Preserves case sensitivity with appropriate suffixes.
/// 
/// # Parameters
/// 
/// * `text` - The original text to mask
/// * `mask_words` - A JavaScript Set containing the words to mask
/// 
/// # Returns
/// 
/// A String with the specified words masked with field placeholders.
#[wasm_bindgen]
pub fn mask_text_with_fields(text: String, mask_words: &Set) -> String {
    text_processor::mask_text_with_fields(text, mask_words)
}

/// Decodes text that was previously masked with field placeholders.
/// 
/// Replaces each field placeholder (FIELD_N) with its corresponding original word.
/// Handles different case variants using the appropriate suffixes.
/// 
/// # Parameters
/// 
/// * `text` - The obfuscated text to decode
/// * `mask_words` - A JavaScript Set containing the original words
/// 
/// # Returns
/// 
/// A String with field placeholders replaced with their original words.
#[wasm_bindgen]
pub fn decode_obfuscated_text(text: String, mask_words: &Set) -> String {
    text_processor::decode_obfuscated_text(text, mask_words)
}

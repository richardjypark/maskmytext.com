mod case_utils;
mod text_processor;
mod utils;

use js_sys::Set;
use wasm_bindgen::prelude::*;

pub use case_utils::{capitalize_first, determine_case_suffix};

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

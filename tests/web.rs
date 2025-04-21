//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use js_sys::Set;
use wasm_bindgen::JsValue;

// Import functions from our crate
use mask_my_text::{greet, mask_text, mask_text_with_fields, decode_obfuscated_text};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_greet() {
    // Call greet and verify the exact message
    let result = greet();
    assert_eq!(
        result,
        "Hello, console log message mask-my-text from Rust!",
        "Greeting message should match expected text"
    );
}

#[wasm_bindgen_test]
fn test_greet_not_empty() {
    // Additional test to verify message is not empty
    let result = greet();
    assert!(
        !result.is_empty(),
        "Greeting message should not be empty"
    );
}

#[wasm_bindgen_test]
fn test_mask_text_basic() {
    // Create a test Set with words to mask
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("secret"));
    mask_words.add(&JsValue::from_str("password"));
    
    let input = "My secret password is confidential.";
    let expected = "My ****** ******** is confidential.";
    
    let result = mask_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Words should be masked with asterisks");
}

#[wasm_bindgen_test]
fn test_mask_text_case_insensitive() {
    // Test case insensitivity
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("secret"));
    
    let input = "This is a Secret that should be SECRET.";
    let expected = "This is a ****** that should be ******.";
    
    let result = mask_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Masking should be case insensitive");
}

#[wasm_bindgen_test]
fn test_mask_text_empty_word() {
    // Test with empty word which should be skipped
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str(""));
    mask_words.add(&JsValue::from_str("password"));
    
    let input = "My password is secure.";
    let expected = "My ******** is secure.";
    
    let result = mask_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Empty words should be skipped");
}

#[wasm_bindgen_test]
fn test_mask_text_with_fields_basic() {
    // Create a test Set with words to mask
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("name"));
    mask_words.add(&JsValue::from_str("email"));
    
    let input = "My name is John and my email is john@example.com.";
    let expected = "My FIELD_1 is John and my FIELD_2 is john@example.com.";
    
    let result = mask_text_with_fields(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Words should be replaced with FIELD_N format");
}

#[wasm_bindgen_test]
fn test_mask_text_with_fields_incremental() {
    // Test incremental field numbers
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("first"));
    mask_words.add(&JsValue::from_str("second"));
    mask_words.add(&JsValue::from_str("third"));
    
    let input = "The first, second, and third items.";
    let expected = "The FIELD_1, FIELD_2, and FIELD_3 items.";
    
    let result = mask_text_with_fields(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Field numbers should increment correctly");
}

#[wasm_bindgen_test]
fn test_mask_text_with_fields_multiple_occurrences() {
    // Test with multiple occurrences of the same word
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("repeat"));
    
    let input = "This repeat will repeat and repeat again.";
    let expected = "This FIELD_1 will FIELD_1 and FIELD_1 again.";
    
    let result = mask_text_with_fields(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Same words should use same field reference");
}

#[wasm_bindgen_test]
fn test_decode_obfuscated_text_basic() {
    // Create a test Set with words to decode
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("John"));
    mask_words.add(&JsValue::from_str("john@example.com"));
    
    let input = "My FIELD_1 is FIELD_1 and my FIELD_2 is FIELD_2.";
    let expected = "My John is John and my john@example.com is john@example.com.";
    
    let result = decode_obfuscated_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "FIELD_N should be replaced with corresponding words");
}

#[wasm_bindgen_test]
fn test_decode_obfuscated_text_empty() {
    // Test with empty text
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("test"));
    
    let input = "";
    let expected = "";
    
    let result = decode_obfuscated_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Empty text should return empty result");
}

#[wasm_bindgen_test]
fn test_decode_obfuscated_text_no_fields() {
    // Test with text that has no fields to replace
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("secret"));
    mask_words.add(&JsValue::from_str("password"));
    
    let input = "This text has no fields to replace.";
    let expected = "This text has no fields to replace.";
    
    let result = decode_obfuscated_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Text without fields should remain unchanged");
}

#[wasm_bindgen_test]
fn test_decode_obfuscated_text_empty_words() {
    // Test with empty words which should be skipped
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str(""));
    mask_words.add(&JsValue::from_str("valid"));
    
    let input = "This FIELD_1 should be replaced.";
    let expected = "This valid should be replaced.";
    
    let result = decode_obfuscated_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Empty words should be skipped during field mapping");
}

#[wasm_bindgen_test]
fn test_mask_and_decode_roundtrip() {
    // Test a full roundtrip: mask with fields and then decode
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("username"));
    mask_words.add(&JsValue::from_str("password"));
    
    let original = "My username is admin and my password is 12345.";
    
    // First mask the text
    let masked = mask_text_with_fields(original.to_string(), &mask_words);
    assert_eq!(
        masked,
        "My FIELD_1 is admin and my FIELD_2 is 12345.",
        "Text should be properly masked with fields"
    );
    
    // Then decode it back
    let decoded = decode_obfuscated_text(masked, &mask_words);
    assert_eq!(
        decoded,
        original,
        "Decoded text should match the original text"
    );
}

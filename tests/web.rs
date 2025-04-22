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
    // Create a test Set with words to mask - email is longer than name, so it will be processed first
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("name"));
    mask_words.add(&JsValue::from_str("email"));
    
    let input = "My name is John and my email is john@example.com.";
    let expected = "My FIELD_2 is John and my FIELD_1 is john@example.com.";
    
    let result = mask_text_with_fields(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Words should be replaced with FIELD_N format based on length order");
}

#[wasm_bindgen_test]
fn test_mask_text_with_fields_incremental() {
    // Test incremental field numbers - words will be ordered by length
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("first"));  // 5 chars
    mask_words.add(&JsValue::from_str("second")); // 6 chars
    mask_words.add(&JsValue::from_str("third"));  // 5 chars
    
    // second (6 chars) gets FIELD_1, first/third (5 chars) get FIELD_2/FIELD_3 in order of appearance
    let input = "The first, second, and third items.";
    let expected = "The FIELD_2, FIELD_1, and FIELD_3 items.";
    
    let result = mask_text_with_fields(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Field numbers should be assigned based on word length order, with same-length words keeping original order");
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
    // Create a test Set with words to decode - email is longer so it will be processed first
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("John"));  // Note: Capitalized in mask list
    mask_words.add(&JsValue::from_str("john@example.com"));  // Note: Lowercase in mask list
    
    // Test all casing variants:
    // - Base field (no suffix) -> lowercase
    // - _F suffix -> First letter capitalized
    // - _A suffix -> ALL CAPS
    let input = "My FIELD_2 is FIELD_2_F and my FIELD_1 is FIELD_1_A.";
    let expected = "My john is John and my john@example.com is JOHN@EXAMPLE.COM.";
    
    let result = decode_obfuscated_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "FIELD_N should be replaced with corresponding words with correct casing");
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
    // 'password' (8 chars) and 'username' (8 chars) are same length
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("username"));
    mask_words.add(&JsValue::from_str("password"));
    
    let original = "My username is admin and my password is 12345.";
    
    // First mask the text - both words are 8 chars, so order is preserved
    let masked = mask_text_with_fields(original.to_string(), &mask_words);
    assert_eq!(
        masked,
        "My FIELD_1 is admin and my FIELD_2 is 12345.",
        "Text should be properly masked with fields based on word length order"
    );
    
    // Then decode it back
    let decoded = decode_obfuscated_text(masked, &mask_words);
    assert_eq!(
        decoded,
        original,
        "Decoded text should match the original text"
    );
}

#[wasm_bindgen_test]
fn test_mask_and_decode_case_preservation() {
    // Test case preservation for different case patterns
    // Words ordered by length: email (5), name (4), id (2)
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("name"));
    mask_words.add(&JsValue::from_str("email"));
    mask_words.add(&JsValue::from_str("id"));
    
    let original = "My Name is john, my EMAIL is test@example.com, and my ID is ABC123.";
    
    // First mask the text - should include case information in fields
    // email (5 chars) gets FIELD_1, name (4 chars) gets FIELD_2, id (2 chars) gets FIELD_3
    let masked = mask_text_with_fields(original.to_string(), &mask_words);
    assert_eq!(
        masked,
        "My FIELD_2_F is john, my FIELD_1_A is test@example.com, and my FIELD_3_A is ABC123.",
        "Text should be masked with case information preserved in field suffixes, ordered by word length"
    );
    
    // Then decode it back - should restore original casing
    let decoded = decode_obfuscated_text(masked, &mask_words);
    assert_eq!(
        decoded,
        original,
        "Decoded text should preserve the original casing of words"
    );
}

#[wasm_bindgen_test]
fn test_case_preservation_variations() {
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("test"));
    
    // Test lowercase
    let lowercase = "this is a test message";
    let masked_lower = mask_text_with_fields(lowercase.to_string(), &mask_words);
    assert_eq!(
        masked_lower,
        "this is a FIELD_1 message",
        "Lowercase word should use base field without suffix"
    );
    
    // Test First Letter Capitalized
    let titlecase = "this is a Test message";
    let masked_title = mask_text_with_fields(titlecase.to_string(), &mask_words);
    assert_eq!(
        masked_title,
        "this is a FIELD_1_F message",
        "Title case word should use _F suffix"
    );
    
    // Test ALL CAPS
    let uppercase = "this is a TEST message";
    let masked_upper = mask_text_with_fields(uppercase.to_string(), &mask_words);
    assert_eq!(
        masked_upper,
        "this is a FIELD_1_A message",
        "Uppercase word should use _A suffix"
    );
    
    // Test decoding preserves all cases
    let decoded_lower = decode_obfuscated_text(masked_lower, &mask_words);
    let decoded_title = decode_obfuscated_text(masked_title, &mask_words);
    let decoded_upper = decode_obfuscated_text(masked_upper, &mask_words);
    
    assert_eq!(decoded_lower, lowercase, "Should preserve lowercase");
    assert_eq!(decoded_title, titlecase, "Should preserve title case");
    assert_eq!(decoded_upper, uppercase, "Should preserve uppercase");
}

#[wasm_bindgen_test]
fn test_mask_text_substring_words() {
    // Test handling of words that are substrings of other words
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("bob"));
    mask_words.add(&JsValue::from_str("bobby"));
    
    let input = "bob and bobby are different names";
    let expected = "*** and ***** are different names";
    
    let result = mask_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Longer words containing shorter mask words should be masked correctly");

    // Test with fields masking as well - bobby (5 chars) gets FIELD_1, bob (3 chars) gets FIELD_2
    let result_fields = mask_text_with_fields(input.to_string(), &mask_words);
    let expected_fields = "FIELD_2 and FIELD_1 are different names";
    assert_eq!(result_fields, expected_fields, "Field masking should handle substring words correctly");
}

#[wasm_bindgen_test]
fn test_decode_obfuscated_text_mask_word_casing() {
    // Test that mask words with different casing are handled correctly
    let mask_words = Set::new(&JsValue::NULL);
    mask_words.add(&JsValue::from_str("Rich"));    // Capitalized in mask list
    mask_words.add(&JsValue::from_str("richard")); // Lowercase in mask list
    
    // Text with various casings of the masked words
    let input = "i want to know which names get removed from the mask, FIELD_2 or FIELD_1 or FIELD_1_F, or FIELD_2, or FIELD_2_F or FIELD_1_A or FIELD_2_A";
    
    // Expected behavior: Base field without suffix should use lowercase,
    // _F suffix should have first letter capitalized, _A suffix should be all uppercase
    let expected = "i want to know which names get removed from the mask, rich or richard or Richard, or rich, or Rich or RICHARD or RICH";
    
    let result = decode_obfuscated_text(input.to_string(), &mask_words);
    assert_eq!(result, expected, "Decoding should properly handle casing regardless of mask word casing");
}

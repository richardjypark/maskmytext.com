//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

// Import greet from our own crate
use mask_my_text::greet;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_greet() {
    // Call greet and verify the exact message
    let result = greet();
    assert_eq!(
        result,
        "Hello, log message mask-my-text from Rust!",
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

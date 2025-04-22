mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::{Array, Set};
use regex::Regex;
use std::collections::HashMap;

// Return the greeting message so it can be tested
#[wasm_bindgen]
pub fn greet() -> String {
    let message = "Hello, console log message mask-my-text from Rust!";
    // Log for visual feedback
    console::log_1(&JsValue::from_str(message));
    message.to_string()
}

#[wasm_bindgen]
pub fn mask_text(text: String, mask_words: &Set) -> String {
    let mut masked_text = text;
    
    // Convert the JS Set to a Vec of strings we can iterate
    let words: Array = Array::from(mask_words);
    let words_len = words.length();
    
    for i in 0..words_len {
        let word_val = words.get(i);
        // JsValue has an as_string method that returns Option<String>
        if let Some(word) = word_val.as_string() {
            if word.is_empty() {
                continue;
            }
            
            // Create a masked version with asterisks matching word length
            let masked = "*".repeat(word.len());
            
            // Carefully escape the word for regex and wrap in word boundary markers if applicable
            let escaped_word = regex::escape(&word);
            
            // Try to create and apply the regex, log any errors but continue processing
            match Regex::new(&format!(r"(?i){}", escaped_word)) {
                Ok(re) => {
                    masked_text = re.replace_all(&masked_text, &masked).to_string();
                },
                Err(e) => {
                    // Log the error but continue with other words
                    console::log_1(&JsValue::from_str(&format!("Regex error for word '{}': {}", word, e)));
                }
            }
        }
    }
    
    masked_text
}

#[wasm_bindgen]
pub fn mask_text_with_fields(text: String, mask_words: &Set) -> String {
    let mut masked_text = text;
    let mut field_counter = 1;
    
    // Convert the JS Set to a Vec of strings we can iterate
    let words: Array = Array::from(mask_words);
    let words_len = words.length();
    
    for i in 0..words_len {
        let word_val = words.get(i);
        // JsValue has an as_string method that returns Option<String>
        if let Some(word) = word_val.as_string() {
            if word.is_empty() {
                continue;
            }
            
            // Create a masked version with FIELD_<number> format
            let masked = format!("FIELD_{}", field_counter);
            field_counter += 1;
            
            // Carefully escape the word for regex and wrap in word boundary markers if applicable
            let escaped_word = regex::escape(&word);
            
            // Try to create and apply the regex, log any errors but continue processing
            match Regex::new(&format!(r"(?i){}", escaped_word)) {
                Ok(re) => {
                    masked_text = re.replace_all(&masked_text, &masked).to_string();
                },
                Err(e) => {
                    // Log the error but continue with other words
                    console::log_1(&JsValue::from_str(&format!("Regex error for word '{}': {}", word, e)));
                }
            }
        }
    }
    
    masked_text
}

#[wasm_bindgen]
pub fn decode_obfuscated_text(text: String, mask_words: &Set) -> String {
    if text.is_empty() {
        return String::new();
    }
    
    // Create mapping of field numbers to words
    let mut field_map = HashMap::new();
    let mut field_counter = 1;
    
    // Convert the JS Set to a Vec of strings we can iterate
    let words: Array = Array::from(mask_words);
    let words_len = words.length();
    
    for i in 0..words_len {
        let word_val = words.get(i);
        if let Some(word) = word_val.as_string() {
            if !word.is_empty() {
                let field = format!("FIELD_{}", field_counter);
                field_map.insert(field, word);
                field_counter += 1;
            }
        }
    }
    
    // Replace all FIELD_X occurrences with their original words
    let mut decoded_text = text;
    for (field, word) in field_map {
        // Create regex for the field
        if let Ok(re) = Regex::new(&field) {
            decoded_text = re.replace_all(&decoded_text, word).to_string();
        } else {
            // Log error but continue with other fields
            console::log_1(&JsValue::from_str(&format!("Regex error for field '{}'", field)));
        }
    }
    
    decoded_text
}

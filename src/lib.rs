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
                    // Collect all matches and their replacements first
                    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
                    
                    for m in re.find_iter(&masked_text) {
                        let matched_word = &masked_text[m.start()..m.end()];
                        let case_suffix = determine_case_suffix(matched_word);
                        let case_masked = format!("{}{}", masked, case_suffix);
                        replacements.push((m.start(), m.end(), case_masked));
                    }
                    
                    // Apply replacements in reverse order
                    for (start, end, replacement) in replacements.into_iter().rev() {
                        masked_text.replace_range(start..end, &replacement);
                    }
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

// Helper function to determine case suffix
fn determine_case_suffix(word: &str) -> &'static str {
    if word.is_empty() {
        return "";
    }
    
    let first_char = word.chars().next().unwrap();
    let is_first_upper = first_char.is_uppercase();
    
    // Check if all characters are uppercase
    let is_all_upper = word.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    
    if is_all_upper && word.len() > 1 {
        "_A" // All uppercase
    } else if is_first_upper {
        "_F" // First letter uppercase
    } else {
        "" // Default case (lowercase)
    }
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
                // Create all possible variants of the field with case information
                let base_field = format!("FIELD_{}", field_counter);
                
                // Map each variant to the appropriate cased version of the word
                field_map.insert(format!("{}_A", base_field), word.to_uppercase());
                field_map.insert(format!("{}_F", base_field), capitalize_first(&word));
                field_map.insert(base_field, word);
                
                field_counter += 1;
            }
        }
    }
    
    // Replace all FIELD_X occurrences with their original words
    let mut decoded_text = text;
    
    // Sort keys by length in descending order to match the most specific patterns first
    let mut keys: Vec<_> = field_map.keys().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));
    
    for key in keys {
        if let Some(word) = field_map.get(key) {
            // Create regex for the field
            if let Ok(re) = Regex::new(&regex::escape(key)) {
                decoded_text = re.replace_all(&decoded_text, word).to_string();
            } else {
                // Log error but continue with other fields
                console::log_1(&JsValue::from_str(&format!("Regex error for field '{}'", key)));
            }
        }
    }
    
    decoded_text
}

// Helper function to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

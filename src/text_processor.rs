/// Text processing module for masking and obfuscating text.
/// 
/// This module contains the core functionality for masking sensitive words
/// in text with various replacement strategies and decoding masked text.

use js_sys::{Array, Set};
use regex::Regex;
use std::collections::HashMap;
use web_sys::console;
use wasm_bindgen::JsValue;

use crate::case_utils::{determine_case_suffix, capitalize_first};

/// Converts a JavaScript Set to a sorted Vec of strings.
/// 
/// # Parameters
/// 
/// * `mask_words` - The JavaScript Set containing words to process
/// 
/// # Returns
/// 
/// A vector of (String, usize) tuples, sorted by word length in descending order
fn set_to_sorted_vec(mask_words: &Set) -> Vec<(String, usize)> {
    let words: Array = Array::from(mask_words);
    let words_len = words.length();
    
    // Early return for empty set
    if words_len == 0 {
        return Vec::new();
    }
    
    let mut word_vec: Vec<(String, usize)> = Vec::with_capacity(words_len as usize);
    
    // Collect words into a Vec with their original indices
    for i in 0..words_len {
        if let Some(word) = words.get(i).as_string() {
            if !word.is_empty() {
                word_vec.push((word, i as usize));
            }
        }
    }
    
    // Sort words by length in descending order, using original index as tiebreaker
    word_vec.sort_by(|a, b| {
        b.0.len().cmp(&a.0.len())
            .then_with(|| a.1.cmp(&b.1))
    });
    
    word_vec
}

/// Creates a regex pattern for matching words with word boundaries.
/// 
/// The pattern is case-insensitive and uses word boundaries to prevent
/// partial matches within larger words.
/// 
/// # Parameters
/// 
/// * `word` - The word to create a regex pattern for
/// 
/// # Returns
/// 
/// A Result containing either the compiled Regex or an error
fn create_word_boundary_regex(word: &str) -> Result<Regex, regex::Error> {
    let escaped_word = regex::escape(word);
    Regex::new(&format!(r"(?i)\b{}\b", escaped_word))
}

/// Logs an error message to the JavaScript console.
/// 
/// # Parameters
/// 
/// * `message` - The error message to log
#[inline]
fn log_error(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// Masks specified words in text with asterisks.
/// 
/// # Parameters
/// 
/// * `text` - The text to process
/// * `mask_words` - A JavaScript Set containing words to mask
/// 
/// # Returns
/// 
/// The processed text with specified words replaced by asterisks
pub fn mask_text(text: String, mask_words: &Set) -> String {
    // Early return for empty text or empty word set
    if text.is_empty() || mask_words.size() == 0 {
        return text;
    }
    
    let mut masked_text = text;
    let word_vec: Vec<String> = set_to_sorted_vec(mask_words)
        .into_iter()
        .map(|(word, _)| word)
        .collect();
    
    // Pre-allocate asterisk masks as a HashMap to avoid repeated allocations
    let mut asterisk_masks: HashMap<usize, String> = HashMap::new();
    
    for word in word_vec {
        // Skip empty words
        if word.is_empty() {
            continue;
        }
        
        // Get or create asterisk mask of right length
        let word_len = word.len();
        let masked = asterisk_masks
            .entry(word_len)
            .or_insert_with(|| "*".repeat(word_len))
            .clone();
        
        // Create and apply the regex
        match create_word_boundary_regex(&word) {
            Ok(re) => {
                masked_text = re.replace_all(&masked_text, &masked).to_string();
            },
            Err(e) => {
                log_error(&format!("Regex error for word '{}': {}", word, e));
            }
        }
    }
    
    masked_text
}

/// Masks specified words in text with field placeholders.
/// 
/// Replaces words with FIELD_N placeholders, preserving case information
/// with appropriate suffixes.
/// 
/// # Parameters
/// 
/// * `text` - The text to process
/// * `mask_words` - A JavaScript Set containing words to mask
/// 
/// # Returns
/// 
/// The processed text with specified words replaced by field placeholders
pub fn mask_text_with_fields(text: String, mask_words: &Set) -> String {
    // Early return for empty text or empty word set
    if text.is_empty() || mask_words.size() == 0 {
        return text;
    }
    
    let mut masked_text = text;
    let mut field_counter = 1;
    
    // Get sorted words
    let word_vec = set_to_sorted_vec(mask_words);
    
    // Create a mapping of words to their field numbers
    let mut word_to_field: HashMap<String, usize> = HashMap::with_capacity(word_vec.len());
    
    for (word, _) in &word_vec {
        if !word_to_field.contains_key(word) {
            word_to_field.insert(word.clone(), field_counter);
            field_counter += 1;
        }
    }
    
    // Process words in sorted order
    for (word, _) in word_vec {
        // Skip empty words
        if word.is_empty() {
            continue;
        }
        
        let field_num = word_to_field.get(&word).unwrap_or(&0);
        if *field_num == 0 {
            log_error(&format!("Could not find field number for word '{}'", word));
            continue;
        }
        
        let masked = format!("FIELD_{}", field_num);
        
        // Try to create and apply the regex
        match create_word_boundary_regex(&word) {
            Ok(re) => {
                // Collect all matches and their replacements first
                let mut replacements: Vec<(usize, usize, String)> = Vec::new();
                
                for m in re.find_iter(&masked_text) {
                    let matched_word = &masked_text[m.start()..m.end()];
                    let case_suffix = determine_case_suffix(matched_word);
                    let case_masked = format!("{}{}", masked, case_suffix);
                    replacements.push((m.start(), m.end(), case_masked));
                }
                
                // Skip processing if no replacements needed
                if replacements.is_empty() {
                    continue;
                }
                
                // Apply replacements in reverse order to prevent invalidating indices
                for (start, end, replacement) in replacements.into_iter().rev() {
                    masked_text.replace_range(start..end, &replacement);
                }
            },
            Err(e) => {
                log_error(&format!("Regex error for word '{}': {}", word, e));
            }
        }
    }
    
    masked_text
}

/// Decodes text that was previously masked with field placeholders.
/// 
/// # Parameters
/// 
/// * `text` - The text with field placeholders to decode
/// * `mask_words` - A JavaScript Set containing the original words
/// 
/// # Returns
/// 
/// The decoded text with field placeholders replaced by their original words
pub fn decode_obfuscated_text(text: String, mask_words: &Set) -> String {
    // Early return for empty text
    if text.is_empty() || mask_words.size() == 0 {
        return text;
    }
    
    // Create mapping of field numbers to words
    let mut field_counter = 1;
    
    // Get sorted words
    let word_vec = set_to_sorted_vec(mask_words);
    
    // Pre-calculate the expected number of keys
    let expected_keys = word_vec.len() * 3; // 3 variants per word
    
    // Create field mappings with pre-allocated capacity
    let mut field_map = HashMap::with_capacity(expected_keys);
    
    for (word, _) in word_vec {
        let base_field = format!("FIELD_{}", field_counter);
        let lowercase_word = word.to_lowercase();
        
        // Map each variant to the appropriate cased version of the word
        field_map.insert(format!("{}_A", base_field), word.to_uppercase());
        field_map.insert(format!("{}_F", base_field), capitalize_first(&lowercase_word));
        field_map.insert(base_field.clone(), lowercase_word);
        
        field_counter += 1;
    }
    
    // Check if any field patterns exist in the text before processing
    let mut contains_field = false;
    for key in field_map.keys() {
        if text.contains(key) {
            contains_field = true;
            break;
        }
    }
    
    // Early return if no fields to replace
    if !contains_field {
        return text;
    }
    
    // Replace all FIELD_X occurrences with their original words
    let mut decoded_text = text;
    
    // Sort keys by length in descending order to match the most specific patterns first
    let mut keys: Vec<_> = field_map.keys().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));
    
    // Compile all regexes up front
    let regexes: Vec<(String, Regex)> = keys
        .into_iter()
        .filter_map(|key| {
            match Regex::new(&regex::escape(key)) {
                Ok(re) => Some((key.clone(), re)),
                Err(_) => {
                    log_error(&format!("Regex error for field '{}'", key));
                    None
                }
            }
        })
        .collect();
    
    // Apply all regex replacements
    for (key, re) in regexes {
        if let Some(word) = field_map.get(&key) {
            decoded_text = re.replace_all(&decoded_text, word).to_string();
        }
    }
    
    decoded_text
} 
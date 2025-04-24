use fancy_regex::Regex;
/// Text processing module for masking and obfuscating text.
///
/// This module contains the core functionality for masking sensitive words
/// in text with various replacement strategies and decoding masked text.
use js_sys::{Array, Set};
use regex::Regex as StdRegex;
use std::cmp::Reverse;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::case_utils::{capitalize_first, determine_case_suffix};

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
    let mut seen_lowercase: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Collect words into a Vec with their original indices, deduplicated case-insensitively
    for i in 0..words_len {
        if let Some(word) = words.get(i).as_string() {
            if !word.is_empty() {
                let lower = word.to_lowercase();
                // skip if we've already seen this lowercase word
                if seen_lowercase.insert(lower) {
                    word_vec.push((word, i as usize));
                }
            }
        }
    }

    // Sort words by length in descending order, using original index as tiebreaker
    word_vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.1.cmp(&b.1)));

    word_vec
}

/// Creates a regex pattern that precisely matches words in various contexts.
///
/// Uses lookahead and lookbehind to match words in camelCase, snake_case,
/// and other compound word formats, while maintaining proper word boundaries.
///
/// # Parameters
///
/// * `word` - The word to create a regex pattern for
///
/// # Returns
///
/// A compiled Regex
fn create_word_boundary_regex(word: &str) -> Regex {
    let escaped_word = regex::escape(word);

    // Pattern to match:
    // 1. Complete standalone word (\bword\b)
    // 2. Word as prefix (\bword(?=[A-Z_\-0-9]))
    // 3. Word as suffix ((?<=[A-Z_\-0-9])word\b)
    // 4. Word in the middle ((?<=[A-Z_\-0-9])word(?=[A-Z_\-0-9]))
    let pattern = format!(
        r"(?i)(?:\b{w}\b|\b{w}(?=[A-Z_\-\d])|(?<=[A-Z_\-\d]){w}\b|(?<=[A-Z_\-\d]){w}(?=[A-Z_\-\d]))",
        w = escaped_word
    );

    // Unwrap is safe here since the pattern is constructed programmatically
    match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(e) => {
            // Log error but provide a fallback pattern that will at least work for whole words
            console::log_1(&JsValue::from_str(&format!("Error creating regex: {}", e)));
            let fallback = format!(r"(?i)\b{}\b", escaped_word);
            Regex::new(&fallback).unwrap_or_else(|_| {
                // This should never happen given the simple pattern
                Regex::new(r"$.^").unwrap() // Regex that never matches
            })
        }
    }
}

/// Creates a regex pattern specifically for matching field patterns in obfuscated text.
///
/// This function handles the special case of adjacent field patterns that may occur
/// when decoding compound words with multiple masked parts.
///
/// # Parameters
///
/// * `field` - The field pattern to match
///
/// # Returns
///
/// A compiled StdRegex
fn create_field_pattern_regex(field: &str) -> StdRegex {
    let escaped_field = regex::escape(field);
    // Create a pattern that doesn't break at word boundaries but matches exactly
    StdRegex::new(&escaped_field).unwrap_or_else(|e| {
        // Log error but provide a fallback pattern that will never match
        console::log_1(&JsValue::from_str(&format!(
            "Error creating field regex: {}",
            e
        )));
        StdRegex::new(r"$.^").unwrap() // Regex that never matches
    })
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

/// Find all matches using fancy-regex and handle errors properly.
///
/// # Parameters
///
/// * `regex` - The fancy-regex to use
/// * `text` - The text to search
/// * `word` - The word being searched (for error reporting)
///
/// # Returns
///
/// A vector of match ranges (start, end)
fn find_all_matches(regex: &Regex, text: &str, word: &str) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();

    // Try to find all matches
    let iter_result = regex.find_iter(text);
    for mtch in iter_result {
        match mtch {
            Ok(m) => {
                matches.push((m.start(), m.end()));
            }
            Err(e) => {
                log_error(&format!("Match error for word '{}': {}", word, e));
            }
        }
    }

    matches
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

        // Create regex and find all matches
        let regex = create_word_boundary_regex(&word);
        let matches = find_all_matches(&regex, &masked_text, &word);

        // Apply replacements in reverse order to prevent invalidating indices
        if !matches.is_empty() {
            for (start, end) in matches.into_iter().rev() {
                masked_text.replace_range(start..end, &masked);
            }
        }
    }

    masked_text
}

/// Masks specified words in text with field placeholders.
///
/// Replaces words with FIELD_N placeholders, preserving case information
/// with appropriate suffixes and maintaining any attached characters.
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
        // Ensure proper handling of case and structure
        let field_num = word_to_field.get(&word).unwrap_or(&0);
        if *field_num == 0 {
            log_error(&format!("Could not find field number for word '{}'", word));
            continue;
        }

        let base_field = format!("FIELD_{}", field_num);

        // Create regex and find all matches
        let regex = create_word_boundary_regex(&word);
        let matches = find_all_matches(&regex, &masked_text, &word);

        // Process each match
        let mut replacements = Vec::with_capacity(matches.len());

        for (start, end) in matches {
            let matched_word = &masked_text[start..end];
            let case_suffix = determine_case_suffix(matched_word);
            let case_masked = format!("{}{}", base_field, case_suffix);
            replacements.push((start, end, case_masked));
        }

        // Apply replacements in reverse order to prevent invalidating indices
        for (start, end, replacement) in replacements.into_iter().rev() {
            masked_text.replace_range(start..end, &replacement);
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
        field_map.insert(
            format!("{}_F", base_field),
            capitalize_first(&lowercase_word),
        );
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

    // First we'll create a complete mapping of all possible field patterns to their replacement words
    let mut complete_field_map = HashMap::new();

    // Add basic field patterns from field_map
    for (field, word) in &field_map {
        complete_field_map.insert(field.clone(), word.clone());
    }

    // Get all possible field numbers
    let field_numbers: Vec<usize> = (1..=field_map.len() / 3).collect();

    // Add compound patterns with underscore
    for i in &field_numbers {
        for j in &field_numbers {
            // Regular to regular
            let pattern = format!("FIELD_{}_FIELD_{}", i, j);
            let base_word_i = field_map
                .get(&format!("FIELD_{}", i))
                .unwrap_or(&String::new())
                .clone();
            let base_word_j = field_map
                .get(&format!("FIELD_{}", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern.clone(), format!("{}_{}", base_word_i, base_word_j));

            // Regular to uppercase
            let pattern_ua = format!("FIELD_{}_FIELD_{}_A", i, j);
            let upper_word_j = field_map
                .get(&format!("FIELD_{}_A", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_ua, format!("{}_{}", base_word_i, upper_word_j));

            // Regular to titlecase
            let pattern_uf = format!("FIELD_{}_FIELD_{}_F", i, j);
            let title_word_j = field_map
                .get(&format!("FIELD_{}_F", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_uf, format!("{}_{}", base_word_i, title_word_j));

            // Uppercase to regular
            let pattern_au = format!("FIELD_{}_A_FIELD_{}", i, j);
            let upper_word_i = field_map
                .get(&format!("FIELD_{}_A", i))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_au, format!("{}_{}", upper_word_i, base_word_j));

            // Uppercase to uppercase
            let pattern_aa = format!("FIELD_{}_A_FIELD_{}_A", i, j);
            complete_field_map.insert(pattern_aa, format!("{}_{}", upper_word_i, upper_word_j));

            // Uppercase to titlecase
            let pattern_af = format!("FIELD_{}_A_FIELD_{}_F", i, j);
            complete_field_map.insert(pattern_af, format!("{}_{}", upper_word_i, title_word_j));

            // Titlecase to regular
            let pattern_fu = format!("FIELD_{}_F_FIELD_{}", i, j);
            let title_word_i = field_map
                .get(&format!("FIELD_{}_F", i))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_fu, format!("{}_{}", title_word_i, base_word_j));

            // Titlecase to uppercase
            let pattern_fa = format!("FIELD_{}_F_FIELD_{}_A", i, j);
            complete_field_map.insert(pattern_fa, format!("{}_{}", title_word_i, upper_word_j));

            // Titlecase to titlecase
            let pattern_ff = format!("FIELD_{}_F_FIELD_{}_F", i, j);
            complete_field_map.insert(pattern_ff, format!("{}_{}", title_word_i, title_word_j));
        }
    }

    // Add direct adjacency patterns (no underscore)
    for i in &field_numbers {
        for j in &field_numbers {
            // Regular to regular
            let pattern = format!("FIELD_{}FIELD_{}", i, j);
            let base_word_i = field_map
                .get(&format!("FIELD_{}", i))
                .unwrap_or(&String::new())
                .clone();
            let base_word_j = field_map
                .get(&format!("FIELD_{}", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern.clone(), format!("{}{}", base_word_i, base_word_j));

            // Regular to uppercase
            let pattern_ua = format!("FIELD_{}FIELD_{}_A", i, j);
            let upper_word_j = field_map
                .get(&format!("FIELD_{}_A", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_ua, format!("{}{}", base_word_i, upper_word_j));

            // Regular to titlecase
            let pattern_uf = format!("FIELD_{}FIELD_{}_F", i, j);
            let title_word_j = field_map
                .get(&format!("FIELD_{}_F", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_uf, format!("{}{}", base_word_i, title_word_j));

            // Uppercase to regular
            let pattern_au = format!("FIELD_{}_AFIELD_{}", i, j);
            let upper_word_i = field_map
                .get(&format!("FIELD_{}_A", i))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_au, format!("{}{}", upper_word_i, base_word_j));

            // Uppercase to uppercase
            let pattern_aa = format!("FIELD_{}_AFIELD_{}_A", i, j);
            complete_field_map.insert(pattern_aa, format!("{}{}", upper_word_i, upper_word_j));

            // Uppercase to titlecase
            let pattern_af = format!("FIELD_{}_AFIELD_{}_F", i, j);
            complete_field_map.insert(pattern_af, format!("{}{}", upper_word_i, title_word_j));

            // Titlecase to regular
            let pattern_fu = format!("FIELD_{}_FFIELD_{}", i, j);
            let title_word_i = field_map
                .get(&format!("FIELD_{}_F", i))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_fu, format!("{}{}", title_word_i, base_word_j));

            // Titlecase to uppercase
            let pattern_fa = format!("FIELD_{}_FFIELD_{}_A", i, j);
            complete_field_map.insert(pattern_fa, format!("{}{}", title_word_i, upper_word_j));

            // Titlecase to titlecase
            let pattern_ff = format!("FIELD_{}_FFIELD_{}_F", i, j);
            complete_field_map.insert(pattern_ff, format!("{}{}", title_word_i, title_word_j));
        }
    }

    // Add hyphen patterns
    for i in &field_numbers {
        for j in &field_numbers {
            // Regular to regular
            let pattern = format!("FIELD_{}-FIELD_{}", i, j);
            let base_word_i = field_map
                .get(&format!("FIELD_{}", i))
                .unwrap_or(&String::new())
                .clone();
            let base_word_j = field_map
                .get(&format!("FIELD_{}", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern.clone(), format!("{}-{}", base_word_i, base_word_j));

            // Other combinations with hyphen following similar pattern as with underscore
            let pattern_ua = format!("FIELD_{}-FIELD_{}_A", i, j);
            let upper_word_j = field_map
                .get(&format!("FIELD_{}_A", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_ua, format!("{}-{}", base_word_i, upper_word_j));

            let pattern_uf = format!("FIELD_{}-FIELD_{}_F", i, j);
            let title_word_j = field_map
                .get(&format!("FIELD_{}_F", j))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_uf, format!("{}-{}", base_word_i, title_word_j));

            let pattern_au = format!("FIELD_{}_A-FIELD_{}", i, j);
            let upper_word_i = field_map
                .get(&format!("FIELD_{}_A", i))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_au, format!("{}-{}", upper_word_i, base_word_j));

            let pattern_fu = format!("FIELD_{}_F-FIELD_{}", i, j);
            let title_word_i = field_map
                .get(&format!("FIELD_{}_F", i))
                .unwrap_or(&String::new())
                .clone();
            complete_field_map.insert(pattern_fu, format!("{}-{}", title_word_i, base_word_j));
        }
    }

    // Replace all field patterns with their original words
    let mut decoded_text = text;

    // Process patterns from longest to shortest to avoid substring conflicts
    let mut keys: Vec<_> = complete_field_map.keys().collect();
    keys.sort_by_key(|b| Reverse(b.len()));

    // Apply all replacements
    for key in keys {
        if let Some(word) = complete_field_map.get(key) {
            let regex = create_field_pattern_regex(key);
            decoded_text = regex.replace_all(&decoded_text, word).to_string();
        }
    }

    decoded_text
}

#![allow(clippy::uninlined_format_args)]
/// Text processing module for masking and obfuscating text.
///
/// This module contains the core functionality for masking sensitive words
/// in text with various replacement strategies and decoding masked text.
use js_sys::{Array, Set};
use regex::{Captures, Regex, RegexBuilder};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::case_utils::{capitalize_first, determine_case_suffix};

#[derive(Debug, Clone)]
struct FieldVariants {
    lowercase: String,
    first_upper: String,
    uppercase: String,
}

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

    if words_len == 0 {
        return Vec::new();
    }

    let mut word_vec: Vec<(String, usize)> = Vec::with_capacity(words_len as usize);
    let mut seen_lowercase: HashSet<String> = HashSet::new();

    for i in 0..words_len {
        if let Some(word) = words.get(i).as_string() {
            if word.is_empty() {
                continue;
            }

            let lowercase = word.to_lowercase();
            if seen_lowercase.insert(lowercase) {
                word_vec.push((word, i as usize));
            }
        }
    }

    word_vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.1.cmp(&b.1)));
    word_vec
}

#[inline]
fn log_error(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

fn build_case_insensitive_regex(words: &[String]) -> Option<Regex> {
    if words.is_empty() {
        return None;
    }

    let pattern = words
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");

    RegexBuilder::new(&format!("(?:{})", pattern))
        .case_insensitive(true)
        .build()
        .ok()
}

fn parse_field_number_prefix(
    text: &str,
    start: usize,
    max_fields: usize,
) -> Option<(usize, usize)> {
    const FIELD_PREFIX: &str = "FIELD_";

    if !text[start..].starts_with(FIELD_PREFIX) {
        return None;
    }

    let bytes = text.as_bytes();
    let mut cursor = start + FIELD_PREFIX.len();
    let mut numeric_value = 0usize;
    let mut matched: Option<(usize, usize)> = None;

    while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
        let digit = (bytes[cursor] - b'0') as usize;
        numeric_value = numeric_value.saturating_mul(10).saturating_add(digit);

        if numeric_value != 0 && numeric_value <= max_fields {
            matched = Some((cursor + 1, numeric_value));
        }

        cursor += 1;
    }

    matched
}

fn parse_field_token<'a>(
    text: &str,
    start: usize,
    field_variants: &'a [FieldVariants],
) -> Option<(usize, &'a str)> {
    const FIELD_PREFIX: &str = "FIELD_";

    let (mut cursor, field_num) = parse_field_number_prefix(text, start, field_variants.len())?;

    // Keep unknown complete tokens like FIELD_100_A unchanged; only partial-decode numeric
    // prefixes when the trailing digits are literal text rather than an explicit case suffix.
    let mut digits_end = start + FIELD_PREFIX.len();
    while digits_end < text.len() && text.as_bytes()[digits_end].is_ascii_digit() {
        digits_end += 1;
    }

    if digits_end > cursor
        && (text[digits_end..].starts_with("_A") || text[digits_end..].starts_with("_F"))
    {
        return None;
    }

    let variants = &field_variants[field_num - 1];
    let mut resolved = variants.lowercase.as_str();

    if text[cursor..].starts_with("_A") {
        resolved = variants.uppercase.as_str();
        cursor += 2;
    } else if text[cursor..].starts_with("_F") {
        let next_token_start = cursor + 1;
        let has_adjacent_decodable_field =
            parse_field_token(text, next_token_start, field_variants).is_some();

        if !has_adjacent_decodable_field {
            resolved = variants.first_upper.as_str();
            cursor += 2;
        }
    }

    Some((cursor, resolved))
}

fn decode_streaming_fields(text: &str, field_variants: &[FieldVariants]) -> String {
    let mut decoded = String::with_capacity(text.len());
    let mut cursor = 0;

    while cursor < text.len() {
        if text[cursor..].starts_with("FIELD_") {
            if let Some((mut next_cursor, replacement)) =
                parse_field_token(text, cursor, field_variants)
            {
                decoded.push_str(replacement);
                cursor = next_cursor;

                loop {
                    if cursor < text.len() && text[cursor..].starts_with("FIELD_") {
                        if let Some((parsed_end, parsed_replacement)) =
                            parse_field_token(text, cursor, field_variants)
                        {
                            decoded.push_str(parsed_replacement);
                            cursor = parsed_end;
                            next_cursor = parsed_end;
                            continue;
                        }
                    }

                    if cursor + 1 < text.len() {
                        let separator = text.as_bytes()[cursor];
                        if (separator == b'_' || separator == b'-')
                            && text[cursor + 1..].starts_with("FIELD_")
                        {
                            if let Some((parsed_end, parsed_replacement)) =
                                parse_field_token(text, cursor + 1, field_variants)
                            {
                                decoded.push(separator as char);
                                decoded.push_str(parsed_replacement);
                                cursor = parsed_end;
                                next_cursor = parsed_end;
                                continue;
                            }
                        }
                    }

                    cursor = next_cursor;
                    break;
                }

                continue;
            }
        }

        let Some(character) = text[cursor..].chars().next() else {
            break;
        };
        decoded.push(character);
        cursor += character.len_utf8();
    }

    decoded
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
    if text.is_empty() || mask_words.size() == 0 {
        return text;
    }

    let ordered_words: Vec<String> = set_to_sorted_vec(mask_words)
        .into_iter()
        .map(|(word, _)| word)
        .collect();

    if ordered_words.is_empty() {
        return text;
    }

    let Some(pattern) = build_case_insensitive_regex(&ordered_words) else {
        log_error("Unable to compile masking regex for asterisks mode.");
        return text;
    };

    let mut asterisk_masks: HashMap<usize, String> = HashMap::new();
    pattern
        .replace_all(&text, |captures: &Captures| {
            let Some(matched) = captures.get(0) else {
                return String::new();
            };

            let length = matched.as_str().len();
            asterisk_masks
                .entry(length)
                .or_insert_with(|| "*".repeat(length))
                .clone()
        })
        .to_string()
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
    if text.is_empty() || mask_words.size() == 0 {
        return text;
    }

    let word_vec = set_to_sorted_vec(mask_words);
    if word_vec.is_empty() {
        return text;
    }

    let ordered_words: Vec<String> = word_vec.iter().map(|(word, _)| word.clone()).collect();

    let Some(pattern) = build_case_insensitive_regex(&ordered_words) else {
        log_error("Unable to compile masking regex for field mode.");
        return text;
    };

    let mut field_by_lowercase: HashMap<String, usize> =
        HashMap::with_capacity(ordered_words.len());
    for (index, word) in ordered_words.iter().enumerate() {
        field_by_lowercase.insert(word.to_lowercase(), index + 1);
    }

    pattern
        .replace_all(&text, |captures: &Captures| {
            let Some(matched) = captures.get(0) else {
                return String::new();
            };

            let matched_word = matched.as_str();
            let field_num = field_by_lowercase
                .get(&matched_word.to_lowercase())
                .copied()
                .unwrap_or(0);

            if field_num == 0 {
                return matched_word.to_string();
            }

            let case_suffix = determine_case_suffix(matched_word);
            format!("FIELD_{}{}", field_num, case_suffix)
        })
        .to_string()
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
    if text.is_empty() || mask_words.size() == 0 {
        return text;
    }

    if !text.contains("FIELD_") {
        return text;
    }

    let word_vec = set_to_sorted_vec(mask_words);
    if word_vec.is_empty() {
        return text;
    }

    let field_variants = word_vec
        .into_iter()
        .map(|(word, _)| {
            let lowercase = word.to_lowercase();
            FieldVariants {
                uppercase: word.to_uppercase(),
                first_upper: capitalize_first(&lowercase),
                lowercase,
            }
        })
        .collect::<Vec<_>>();

    decode_streaming_fields(&text, &field_variants)
}

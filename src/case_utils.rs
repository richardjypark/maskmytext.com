/// Utility functions for handling text case sensitivity

/// Determines the case suffix for a given word.
/// 
/// This function analyzes the case pattern of a word and returns
/// an appropriate suffix to indicate its case style:
/// - "_A" for ALL UPPERCASE words (length > 1)
/// - "_F" for First letter uppercase words
/// - "" (empty string) for lowercase words
/// 
/// # Parameters
/// 
/// * `word` - The word to analyze
/// 
/// # Returns
/// 
/// A static string representing the case suffix.
pub fn determine_case_suffix(word: &str) -> &'static str {
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

/// Capitalizes the first letter of a string.
/// 
/// # Parameters
/// 
/// * `s` - The string to capitalize
/// 
/// # Returns
/// 
/// A new string with the first letter capitalized.
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
} 
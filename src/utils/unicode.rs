use crate::error::Result;
use crate::error::SourisError;
use unicode_normalization::UnicodeNormalization;

pub fn normalize_nfc(s: &str) -> String {
    s.nfc().collect()
}

pub fn normalize_nfd(s: &str) -> String {
    s.nfd().collect()
}

pub fn strip_bom(s: &str) -> &str {
    s.strip_prefix('\u{feff}').unwrap_or(s)
}

pub fn normalize_for_compare(s: &str) -> String {
    normalize_nfc(s).to_lowercase()
}

pub fn safe_string(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec())
        .map_err(|e| SourisError::UnicodeError(format!("Invalid UTF-8: {}", e)))
}

pub fn safe_string_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

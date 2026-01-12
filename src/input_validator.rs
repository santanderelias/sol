use unicode_normalization::UnicodeNormalization;

pub fn normalize_input(input: &str) -> String {
    input
        .nfc()
        .collect::<String>()
        .trim()
        .to_lowercase()
}

pub fn is_correct_answer(user_input: &str, japanese: &str, romanji: &str, kanji: Option<&str>) -> bool {
    let normalized_input = normalize_input(user_input);
    let normalized_japanese = normalize_input(japanese);
    let normalized_romanji = normalize_input(romanji);

    // Accept Japanese (hiragana/katakana), Romanji, or Kanji
    if normalized_input == normalized_japanese || normalized_input == normalized_romanji {
        return true;
    }
    
    // Check kanji if provided
    if let Some(k) = kanji {
        let normalized_kanji = normalize_input(k);
        if normalized_input == normalized_kanji {
            return true;
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japanese_input() {
        assert!(is_correct_answer("ねこ", "ねこ", "neko", Some("猫")));
        assert!(is_correct_answer("ネコ", "ねこ", "neko", Some("猫"))); // Different script
    }

    #[test]
    fn test_romanji_input() {
        assert!(is_correct_answer("neko", "ねこ", "neko", Some("猫")));
        assert!(is_correct_answer("NEKO", "ねこ", "neko", Some("猫"))); // Case insensitive
    }

    #[test]
    fn test_kanji_input() {
        assert!(is_correct_answer("猫", "ねこ", "neko", Some("猫")));
        assert!(is_correct_answer("犬", "いぬ", "inu", Some("犬")));
    }

    #[test]
    fn test_incorrect_input() {
        assert!(!is_correct_answer("inu", "ねこ", "neko", Some("猫")));
        assert!(!is_correct_answer("cat", "ねこ", "neko", Some("猫")));
    }

    #[test]
    fn test_whitespace_handling() {
        assert!(is_correct_answer("  neko  ", "ねこ", "neko", Some("猫")));
        assert!(is_correct_answer("  ねこ  ", "ねこ", "neko", Some("猫")));
        assert!(is_correct_answer("  猫  ", "ねこ", "neko", Some("猫")));
    }
}

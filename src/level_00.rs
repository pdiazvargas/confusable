use anyhow::Result;

/// Holds the information of a single row from the confusable data set. A row has the following format:
/// FF21 ; 0041 ; MA # ( Ａ → A ) FULLWIDTH LATIN CAPITAL LETTER A → LATIN CAPITAL LETTER A # →А→
/// 1      2      3
/// 1. The confusable character as unicode.
/// 2. The replacement character as unicode; this could be a set of multiple characters
/// 3. A comment that shows the replacement in a human readable format.
#[derive(Debug, PartialEq, Eq)]
pub struct ConfusableRow {
    confusable: char,
    replacement: Vec<char>,
    comment: String,
}

impl ConfusableRow {
    pub fn new(confusable: char, replacement: Vec<char>, comment: String) -> Self {
        Self {
            confusable,
            replacement,
            comment,
        }
    }
}

impl From<&str> for ConfusableRow {
    fn from(value: &str) -> Self {
        todo!("implement here")
    }
}

/// Convert the hex representation of a unicode character to a char.
/// hex: 0039 -> Ok(9)
fn parse_unicode_code_point(_hex: &str) -> Result<char> {
    // convert given string to hex (u32)
    // convert the result to char
    Ok('9')
}

#[cfg(test)]
mod test_level_0 {
    use super::*;

    #[test]
    fn can_convert_unicode_to_char() {
        assert_eq!(parse_unicode_code_point("0039").unwrap(), '9');
        assert_eq!(parse_unicode_code_point("   0039").unwrap(), '9');
        assert_eq!(parse_unicode_code_point("0021").unwrap(), '!');
        assert_eq!(parse_unicode_code_point("0021     ").unwrap(), '!');
        assert_eq!(parse_unicode_code_point("A60E").unwrap(), '꘎');
    }

    #[test]
    fn parses_unicode_with_invalid_values() {
        let err = parse_unicode_code_point("P039").expect_err("should fail");
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    /// Implement the From trait for ConfusableRow so that we can parse a single row from the confusable
    /// table. For now unwrap to your hearts content. We will use a different trait to handle errors.
    #[test]
    fn can_parse_a_valid_confusable_row() {
        let row = "1D7D7 ;	0039 ;	MA	# ( 𝟗 → 9 ) MATHEMATICAL BOLD DIGIT NINE → DIGIT NINE	#";

        let confusable = ConfusableRow::from(row);

        assert_eq!(confusable.confusable, '\u{1D7D7}');
        assert_eq!(confusable.replacement, vec!['\u{0039}']);
        assert_eq!(
            &confusable.comment,
            "MA	# ( 𝟗 → 9 ) MATHEMATICAL BOLD DIGIT NINE → DIGIT NINE	#"
        )
    }

    #[test]
    fn can_parse_confusable_row_multi_byte_replacement() {
        let row = "0192 ;	0066 0326 ;	MA	# ( ƒ → f̦ ) LATIN SMALL LETTER F WITH HOOK → LATIN SMALL LETTER F, COMBINING COMMA BELOW	# →f̡→";

        let confusable = ConfusableRow::from(row);

        assert_eq!(confusable.confusable, '\u{0192}');
        assert_eq!(confusable.replacement, vec!['\u{0066}', '\u{0326}']);
    }
}

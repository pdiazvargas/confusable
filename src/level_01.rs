use std::io::{self, BufRead};

// No longer using anyhow
//use anyhow::Result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parser error: {0}")]
    ParserError(String),
}

/// Introduce a new structure over the Vec<char>
#[derive(Debug)]
pub struct UnicodePoint(Vec<char>);

#[derive(Debug)]
pub struct ConfusableRow {
    confusable: char,
    replacement: UnicodePoint,
}

impl ConfusableRow {
    pub fn new<R: Into<UnicodePoint>>(confusable: char, replacement: R) -> Self {
        Self {
            confusable,
            replacement: replacement.into(),
        }
    }
}

/// New trait, similar to the From trait, but with error handling.
/// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
impl TryFrom<&str> for ConfusableRow {
    type Error = Error;

    /// Parse a single line from the confusable table data and return a new `ConfusableRow`. If
    /// the parsing does not work, return an error.
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        todo!("Update the implementation from level 0 to handler errors");
    }
}

/// Store a list of the confusable character and the entry they can be replaced with.
#[derive(Debug)]
pub struct TableData {
    data: Vec<ConfusableRow>,
}

impl TableData {
    /// Parse each line of the confusables.txt file and populate the override map.
    /// line: FF21 ; 0041 ; MA # ( Ａ → A ) FULLWIDTH LATIN CAPITAL LETTER A → LATIN CAPITAL LETTER A # →А→
    ///       1         2       3
    /// 1. confusable character
    /// 2. replacement character
    /// 3. comment that gets ignored.
    pub fn parse(source_data: &[u8]) -> Result<Self, Error> {
        let mut data = Vec::new();
        let reader = io::BufReader::new(source_data);

        for _line in reader.lines() {
            todo!("Implement this");
        }

        Ok(TableData { data })
    }
}

/// Convert the hex representation of a unicode character to a char.
/// hex: 0039 -> Ok(9)
fn parse_unicode_code_point(_hex: &str) -> Result<char, Error> {
    todo!("Update your implemantation from level 0 to relay errors");
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
    fn can_parse_a_valid_confusable_row() {
        let row = "1D7D7 ;	0039 ;	MA	# ( 𝟗 → 9 ) MATHEMATICAL BOLD DIGIT NINE → DIGIT NINE	#";

        let confusable = ConfusableRow::try_from(row).expect("Parsing to succeed");

        // TODO: Implement the From<char> for UnicodePoint
        assert_eq!(confusable.confusable, '\u{1D7D7}');
        assert_eq!(confusable.replacement, UnicodePoint::from('\u{0039}'));
        // The same as:
        assert_eq!(confusable.replacement, '\u{0039}'.into());
    }

    #[test]
    fn can_parse_with_multi_byte_replacement() {
        let row = "1481 ;	0062 0307 00B7 ;	MA	# ( ᒁ → ḃ· ) CANADIAN SYLLABICS WEST-CREE KWAA → LATIN SMALL LETTER B, COMBINING DOT ABOVE, MIDDLE DOT	# →ᑳᐧ→";

        let confusable = ConfusableRow::try_from(row).expect("Parsing to succeed");

        assert_eq!(confusable.confusable, '\u{1481}');
        assert_eq!(
            confusable.replacement,
            UnicodePoint(Vec::from(['\u{0062}', '\u{0307}', '\u{00B7}']))
        );
    }

    #[test]
    fn can_create_unicode_point_from_char_iter() {
        // TODO: FromIterator<char> for UnicodePoint
        let point = UnicodePoint(vec!['\u{0062}', '\u{0307}', '\u{00B7}']);

        let point_from_iter = "0062 0307 00B7"
            .split(" ")
            .map(parse_unicode_code_point)
            .collect::<Result<UnicodePoint, Error>>()
            .unwrap();

        assert_eq!(point, point_from_iter);
    }

    #[test]
    fn fails_to_parse_invalid_line() {
        let row = "1481 0062 0307 00B7 MA	# ( ᒁ → ḃ· )";

        let error = ConfusableRow::try_from(row).expect_err("should fail to parse");

        assert_eq!(error.to_string(), "Parser error: Invalid confusable line");
    }

    #[test]
    fn with_invalid_unicode_digit() {
        let row = "P481 ;	0062 0307 00B7 ;	MA	# ( ᒁ → ḃ· )";

        let error = ConfusableRow::try_from(row).expect_err("Should fail to parse");

        // The error message we receive directly from: `u32::from_str_radix`
        assert_eq!(error.to_string(), "invalid digit found in string")
    }

    #[test]
    fn can_parse_multiple_lines() {
        let source = "# confusables.txt
        # Date: 2023-08-11, 17:46:40 GMT
        # © 2023 Unicode®, Inc.
        # Unicode and the Unicode Logo are registered trademarks of Unicode, Inc. in the U.S. and other countries.
        # For terms of use, see https://www.unicode.org/terms_of_use.html
        #
        # Unicode Security Mechanisms for UTS #39
        # Version: 15.1.0
        #
        # For documentation and usage, see https://www.unicode.org/reports/tr39
        #
        05AD ;	0596 ;	MA	# ( ֭ → ֖ ) HEBREW ACCENT DEHI → HEBREW ACCENT TIPEHA	#

        05AE ;	0598 ;	MA	# ( ֮ → ֘ ) HEBREW ACCENT ZINOR → HEBREW ACCENT ZARQA	#

        05A8 ;	0599 ;	MA	# ( ֨ → ֙ ) HEBREW ACCENT QADMA → HEBREW ACCENT PASHTA	#

        05A4 ;	059A ;	MA	# ( ֤ → ֚ ) HEBREW ACCENT MAHAPAKH → HEBREW ACCENT YETIV	#";

        let table = TableData::parse(source.as_bytes()).expect("parsing to succeed");

        // Implement the Deref trait for TableData in order to iterate over the records it holds.
        // https://doc.rust-lang.org/std/ops/trait.Deref.html
        let confusables = &table.iter().collect::<Vec<_>>();

        assert_eq!(confusables.len(), 4);

        // Remember:
        // ConfusableRow::new<R: Into<UnicodePoint>>(confusable: char, replacement: R)
        // ConfusableRow::new('\u{05AD}', '\u{0596}')
        assert_eq!(*confusables[0], ConfusableRow::new('\u{05AD}', '\u{0596}'));
    }
}

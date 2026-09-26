use aho_corasick::AhoCorasick;

use crate::table::{ACCENT, EXCEPTION, LETTER};

mod table;

pub trait Iso843 {
    fn iso843_transliterate(&self) -> String;
}

impl Iso843 for &str {
    /// Transliterates Greek characters to Latin characters according to ISO 843:1997 Type 1,
    /// leaving non-Greek characters unchanged.
    ///
    /// Note that this only handles Unicode NFC normalized strings.
    /// For other formats, you can use the [`unicode-normalization`](https://crates.io/crates/unicode-normalization) crate to normalize the string first.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use iso_843::Iso843 as _;
    ///
    /// assert_eq!("Hello, world!".iso843_transliterate(), "Hello, world!");
    /// assert_eq!("Γεια σου, κόσμε!".iso843_transliterate(), "Geia sou, kósme!");
    /// ```
    fn iso843_transliterate(&self) -> String {
        let from = self.as_bytes();
        let mut to = vec![];

        AhoCorasick::new(EXCEPTION[0])
            .unwrap()
            .try_stream_replace_all(&*from, &mut to, &EXCEPTION[1])
            .unwrap();

        let from = to;
        let mut to = vec![];

        AhoCorasick::new(ACCENT[0])
            .unwrap()
            .try_stream_replace_all(&*from, &mut to, &ACCENT[1])
            .unwrap();

        let from = to;
        let mut to = vec![];

        AhoCorasick::new(LETTER[0])
            .unwrap()
            .try_stream_replace_all(&*from, &mut to, &LETTER[1])
            .unwrap();

        String::from_utf8(to).unwrap()
    }
}

// note that some of these tests are AI-generated
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_letter() {
        assert_eq!(
            "Α Β Γ Δ Ε Ζ Η Θ Ι Κ Λ Μ Ν Ξ Ο Π Ρ Σ Τ Υ Φ Χ Ψ Ω".iso843_transliterate(),
            "A V G D E Z Ī TH I K L M N X O P R S T Y F CH PS Ō"
        );

        assert_eq!(
            "α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω".iso843_transliterate(),
            "a v g d e z ī th i k l m n x o p r s t y f ch ps ō"
        );
    }

    #[test]
    fn final_sigma() {
        assert_eq!("ΣΟΣΟΣ".iso843_transliterate(), "SOSOS");

        assert_eq!("σος".iso843_transliterate(), "sos");

        assert_eq!("Σίσυφος".iso843_transliterate(), "Sísyfos");
    }

    #[test]
    fn multi_letter() {
        assert_eq!("ΘΧΨ".iso843_transliterate(), "THCHPS");

        assert_eq!("θχψ".iso843_transliterate(), "thchps");
    }

    #[test]
    fn preserve_case() {
        assert_eq!("Αθήνα".iso843_transliterate(), "Athī́na");

        assert_eq!("ΘΕΣΣΑΛΟΝΙΚΗ".iso843_transliterate(), "THESSALONIKĪ");

        assert_eq!("θεσσαλονίκη".iso843_transliterate(), "thessaloníkī");
    }

    #[test]
    fn preserve_accent() {
        assert_eq!("ά".iso843_transliterate(), "á");
        assert_eq!("έ".iso843_transliterate(), "é");
        assert_eq!("ή".iso843_transliterate(), "ī́");
        assert_eq!("ί".iso843_transliterate(), "í");
        assert_eq!("ό".iso843_transliterate(), "ó");
        assert_eq!("ύ".iso843_transliterate(), "ý");
        assert_eq!("ώ".iso843_transliterate(), "ṓ");

        assert_eq!("ΆΈΉΊΌΎΏ".iso843_transliterate(), "ÁÉĪ́ÍÓÝṒ");
    }

    #[test]
    fn preserves_dialytika() {
        assert_eq!("ϊ".iso843_transliterate(), "ï");
        assert_eq!("ϋ".iso843_transliterate(), "ÿ");

        assert_eq!("Ϊ".iso843_transliterate(), "Ï");
        assert_eq!("Ϋ".iso843_transliterate(), "Ÿ");
    }

    #[test]
    fn accent_and_dialytika_together() {
        assert_eq!("ΐ".iso843_transliterate(), "ḯ");
        assert_eq!("ΰ".iso843_transliterate(), "ÿ́");
    }

    #[test]
    fn handle_au_exception() {
        assert_eq!("αυ".iso843_transliterate(), "au");
        assert_eq!("ΑΥ".iso843_transliterate(), "AU");
        assert_eq!("Αυ".iso843_transliterate(), "Au");
        assert_eq!("αΥ".iso843_transliterate(), "aU");
    }

    #[test]
    fn handle_eu_exception() {
        assert_eq!("ευ".iso843_transliterate(), "eu");
        assert_eq!("ΕΥ".iso843_transliterate(), "EU");
        assert_eq!("Ευ".iso843_transliterate(), "Eu");
        assert_eq!("εΥ".iso843_transliterate(), "eU");
    }

    #[test]
    fn handle_ou_exception() {
        assert_eq!("ου".iso843_transliterate(), "ou");
        assert_eq!("ΟΥ".iso843_transliterate(), "OU");
        assert_eq!("Ου".iso843_transliterate(), "Ou");
        assert_eq!("οΥ".iso843_transliterate(), "oU");
    }

    #[test]
    fn exception_with_accent() {
        assert_eq!("αύ".iso843_transliterate(), "aú");
        assert_eq!("εύ".iso843_transliterate(), "eú");
        assert_eq!("ού".iso843_transliterate(), "oú");

        assert_eq!("Αύ".iso843_transliterate(), "Aú");
        assert_eq!("Εύ".iso843_transliterate(), "Eú");
        assert_eq!("Ού".iso843_transliterate(), "Oú");

        assert_eq!("ΑΎ".iso843_transliterate(), "AÚ");
        assert_eq!("ΕΎ".iso843_transliterate(), "EÚ");
        assert_eq!("ΟΎ".iso843_transliterate(), "OÚ");
    }

    #[test]
    fn diaeresis_prevents_exception() {
        // The diaeresis means that υ is not part of the αυ/ευ/ου
        // double-vowel combination.
        assert_eq!("αϋ".iso843_transliterate(), "aÿ");
        assert_eq!("εϋ".iso843_transliterate(), "eÿ");
        assert_eq!("οϋ".iso843_transliterate(), "oÿ");

        assert_eq!("ΑΫ".iso843_transliterate(), "AŸ");
    }

    #[test]
    fn does_not_apply_double_vowel_rule_to_unrelated_sequences() {
        assert_eq!("αβ".iso843_transliterate(), "av");
        assert_eq!("εβ".iso843_transliterate(), "ev");
        assert_eq!("οβ".iso843_transliterate(), "ov");

        // υ on its own is transliterated as y.
        assert_eq!("υ".iso843_transliterate(), "y");
        assert_eq!("Υ".iso843_transliterate(), "Y");
    }

    #[test]
    fn non_greek_char_unchanged() {
        assert_eq!(
            "Hello, κόσμε! 123 — test@example.com :)".iso843_transliterate(),
            "Hello, kósme! 123 — test@example.com :)"
        );
    }

    #[test]
    fn latin_char_unchanged() {
        assert_eq!("ABC abc XYZ xyz".iso843_transliterate(), "ABC abc XYZ xyz");
    }

    #[test]
    fn digit_and_punctuation_unchanged() {
        assert_eq!(
            "123 42.5 +-=*/!?.,;:()[]{}<>".iso843_transliterate(),
            "123 42.5 +-=*/!?.,;:()[]{}<>"
        );
    }

    #[test]
    fn whitespace_unchanged() {
        assert_eq!(
            "\tΕλλάδα\nΑθήνα  κόσμος\r\n".iso843_transliterate(),
            "\tElláda\nAthī́na  kósmos\r\n"
        );
    }

    #[test]
    fn real_modern_greek_words() {
        assert_eq!("Ελλάδα".iso843_transliterate(), "Elláda");
        assert_eq!("Ελληνικά".iso843_transliterate(), "Ellīniká");
        assert_eq!("Αθήνα".iso843_transliterate(), "Athī́na");

        // Type I keeps ευ/αυ/ου as eu/au/ou and preserves tonos on the accented vowel.
        assert_eq!("ευχαριστώ".iso843_transliterate(), "eucharistṓ");
        assert_eq!("ταύρος".iso843_transliterate(), "taúros");
        assert_eq!("ούρα".iso843_transliterate(), "oúra");
        assert_eq!("αυτοκίνητο".iso843_transliterate(), "autokínīto");
    }

    #[test]
    fn exception_inside_words() {
        assert_eq!("ταύρος".iso843_transliterate(), "taúros");
        assert_eq!("ευχή".iso843_transliterate(), "euchī́");
        assert_eq!("ούρα".iso843_transliterate(), "oúra");
        assert_eq!("αυτοκίνητο".iso843_transliterate(), "autokínīto");
    }

    #[test]
    fn empty() {
        assert_eq!("".iso843_transliterate(), "");
    }

    #[test]
    fn only_non_greek() {
        let input = "Rust 2026! 🦀";
        assert_eq!(input.iso843_transliterate(), input);
    }
}

mod table;
#[cfg(test)]
mod test;

use aho_corasick::AhoCorasick;

use crate::table::{ACCENT, EXCEPTION, LETTER};

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

impl Iso843 for String {
    fn iso843_transliterate(&self) -> String {
        self.as_str().iso843_transliterate()
    }
}

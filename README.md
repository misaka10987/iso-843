# iso-843

Rust Implementation (Lib / CLI) of the ISO 843 Greek Letter Transcription Standard.

## Usage

**Library**

```rust
use iso_843::Iso843 as _;

assert_eq!("Hello, world!".iso843_transliterate(), "Hello, world!");
assert_eq!("Γεια σου, κόσμε!".iso843_transliterate(), "Geia sou, kósme!");
```

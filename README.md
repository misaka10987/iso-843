# iso-843

Rust implementation (lib / CLI) of the ISO 843 Greek letter transcription standard.

## Install

```shell
cargo install iso-843
```

## Usage

**Library**

```rust
use iso_843::Iso843 as _;

assert_eq!("Hello, world!".iso843_transliterate(), "Hello, world!");
assert_eq!("Γεια σου, κόσμε!".iso843_transliterate(), "Geia sou, kósme!");
```

**CLI**

```shell
iso-843 "Γεια σου, κόσμε!" # "Geia sou, kósme!"

echo "Γεια σου, κόσμε!" > /tmp/greek.txt && iso-843 -i /tmp/greek.txt -o /tmp/latin.txt && cat /tmp/latin.txt # "Geia sou, kósme!"
```

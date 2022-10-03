# A crate to word list encode IDs

This crate can be used to transform `u32` values (IDs) to user-friendly word triples.

The word list is based on [the EFF large word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt)
which can encode nearly 13 bits. Therefore, the base for this crate is 12 bits, which effectively uses only `4096`
items from the list. To encode a `u32` value, three words are required.

The wordlist is statically compiled into the library and cannot be changed.

## Build

```
cargo build
```
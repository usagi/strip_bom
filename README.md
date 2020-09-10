# strip_bom

Add a simple BOM striping feature for `str` and `String`.

## Usage

```rust
use str_strip_bom::*;
```

```rust
// Or std::fs::read_to_string, surf::get, ...
let my_string: Vec<u8> = vec![ 0xefu8, 0xbb, 0xbf, 0xf0, 0x9f, 0x8d, 0xa3 ];
let my_string: String  = String::from_utf8( my_string ).unwrap();

// In this time, my_string has the BOM => true 🍣
println!( "{} {}", my_string.starts_with("\u{feff}"), &my_string );

// Strip BOM
let my_string: &str = my_string.strip_bom();

// my_string (slice) has not the BOM => false 🍣
println!( "{} {}", my_string.starts_with("\u{feff}"), &my_string );
```

## Motivation

1. I author wanted **a simple and lightweight BOM stripper for only `str` and `String`**, not for byte stream or the other of UTF-8 such as UTF-16 or UTF-32.
2. Because, for example, `serde` and `serde_json` has no BOM supporting then it will be fail if I put a UTF-8 BOM source.
3. The rust standard, `str` and `String`s will not support a BOM stripping features.; See also <https://github.com/rust-lang/rfcs/issues/2428>.


## Reference

- <https://tools.ietf.org/html/rfc3629>; RFC3269 "UTF-8, a transformation format of ISO 10646" §6. Byte order mark (BOM)

## License

- [MIT](LICENSE)

## Author

- USAGI.NETWORK / Usagi Ito <https://usagi.network>

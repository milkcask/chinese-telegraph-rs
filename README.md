# 🀄📻 chinese-telegraph

[![crates.io](https://img.shields.io/crates/v/chinese-telegraph.svg)](https://crates.io/crates/chinese-telegraph)
[![docs.rs](https://img.shields.io/docsrs/chinese-telegraph)](https://docs.rs/chinese-telegraph)
[![CI](https://github.com/milkcask/chinese-telegraph-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/milkcask/chinese-telegraph-rs/actions/workflows/rust.yml)
[![license](https://img.shields.io/crates/l/chinese-telegraph.svg)](#license)

Convert Unicode Chinese characters to [Chinese telegraph codes](https://en.wikipedia.org/wiki/Chinese_telegraph_code)
(中文電碼, also known as Chinese commercial code or standard telegraph code) —
the four-digit numerical codes historically used to transmit Chinese text over
telegraph, and still used today in some identity documents and immigration
forms.

Both the Traditional Chinese (Taiwan) and Simplified Chinese code tables are
supported. Lookups are compile-time [perfect hash maps](https://crates.io/crates/phf),
so there is no runtime table construction and no heap allocation.

## Installation

```sh
cargo add chinese-telegraph
```

## Usage

```rust
use chinese_telegraph::{to_telegraph, to_telegraph_str, Table};

// Look up a Traditional Chinese character
assert_eq!(to_telegraph("這", Table::TW), Some(6638));

// Look up a Simplified Chinese character
assert_eq!(to_telegraph("这", Table::CN), Some(6638));

// Search both tables (Traditional first, then Simplified)
assert_eq!(to_telegraph("一", Table::Both), Some(1));

// Unknown characters return None
assert_eq!(to_telegraph("🦀", Table::Both), None);

// Format as the conventional 4-digit code — no heap allocation
assert_eq!(to_telegraph_str("一", Table::Both).unwrap(), "0001");
```

The standard conversion traits are also supported via `TelegraphCode`.
Converting with `try_from`/`try_into` searches both tables (`Table::Both`:
Traditional first, then Simplified); use `TelegraphCode::lookup` to search a
specific table:

```rust
use chinese_telegraph::TelegraphCode;

// Searches the Traditional table first, then the Simplified table
let code = TelegraphCode::try_from('一').unwrap();
let num: usize = code.into();
assert_eq!(num, 1);
assert_eq!(code.to_code_str(), "0001");
```

The input must be exactly one character; passing a longer string returns
`None`. To convert a sentence, iterate over its characters:

```rust
use chinese_telegraph::{Table, TelegraphCode};

let codes: Vec<_> = "電報"
    .chars()
    .filter_map(|c| TelegraphCode::lookup(c, Table::TW))
    .map(|code| code.to_code_str())
    .collect();
assert_eq!(codes, ["7193", "1032"]);
```

## `no_std` support

The crate is `no_std`-compatible and allocation-free at its core: lookups use
compile-time perfect hash maps, and four-digit formatting with
`to_telegraph_str` returns a `CodeStr` that stores the digits inline and
dereferences to `&str` — no `String`, no heap.

All feature flags are **enabled by default**:

- `telegraph-code` — the `TelegraphCode` type with its standard conversion
  traits, and the `NoTelegraphCode` error
- `code-str` — the `CodeStr` inline four-digit string type and
  `to_telegraph_str`
- `std` — `to_telegraph_string`, which formats codes as 4-digit `String`s,
  and (with `telegraph-code`) a `std::error::Error` implementation for
  `NoTelegraphCode`

Everything except `std` is `no_std`-compatible. To use the crate in a
`no_std` environment, disable default features:

```toml
[dependencies]
chinese-telegraph = { version = "0.3", default-features = false, features = [
    "telegraph-code",
    "code-str",
] }
```

## Minimum supported Rust version

Rust 1.66. The MSRV is verified in CI and a bump is considered a breaking
change.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option. The telegraph code tables themselves are derived from public
domain data.

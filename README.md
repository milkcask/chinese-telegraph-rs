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
use chinese_telegraph::{to_telegraph, to_telegraph_string, Table};

// Look up a Traditional Chinese character
assert_eq!(to_telegraph("這", Table::TW), Some(6638));

// Look up a Simplified Chinese character
assert_eq!(to_telegraph("这", Table::CN), Some(6638));

// Search both tables (Traditional first, then Simplified)
assert_eq!(to_telegraph("一", Table::Both), Some(1));

// Unknown characters return None
assert_eq!(to_telegraph("🦀", Table::Both), None);

// Format as the conventional 4-digit code (requires the `std` feature)
assert_eq!(to_telegraph_string("一", Table::Both), Some("0001".to_string()));
```

The input must be exactly one character; passing a longer string returns
`None`. To convert a sentence, iterate over its characters:

```rust
use chinese_telegraph::{to_telegraph_string, Table};

let codes: Vec<_> = "電報"
    .chars()
    .filter_map(|c| to_telegraph_string(&c.to_string(), Table::TW))
    .collect();
assert_eq!(codes, ["7193", "1032"]);
```

## `no_std` support

The crate is `no_std` by default. The `std` feature (enabled by default) adds
`to_telegraph_string`, which formats codes as 4-digit `String`s. To use the
crate in a `no_std` environment:

```toml
[dependencies]
chinese-telegraph = { version = "0.2", default-features = false }
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

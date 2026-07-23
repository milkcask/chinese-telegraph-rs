#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text x='28' y='88' font-size='68'>📻</text><text x='4' y='62' font-size='68'>🀄</text></svg>"
)]
#![doc(
    html_favicon_url = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text x='28' y='88' font-size='68'>📻</text><text x='4' y='62' font-size='68'>🀄</text></svg>"
)]
#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Convert Unicode Chinese characters to [Chinese telegraph codes] (中文電碼).
//!
//! Chinese telegraph codes — also known as Chinese commercial codes or
//! standard telegraph codes — are four-digit numerical codes historically
//! used to transmit Chinese text over telegraph, and still used today in
//! some identity documents and immigration forms.
//!
//! Both the Traditional Chinese (Taiwan) and Simplified Chinese code tables
//! are included. Lookups use compile-time perfect hash maps, so there is no
//! runtime table construction and no heap allocation.
//!
//! [Chinese telegraph codes]: https://en.wikipedia.org/wiki/Chinese_telegraph_code
//!
//! # Usage
//!
//! ```rust
//! use chinese_telegraph::{to_telegraph, to_telegraph_str, Table};
//!
//! // Look up a Traditional Chinese character
//! assert_eq!(to_telegraph("這", Table::TW), Some(6638));
//!
//! // Look up a Simplified Chinese character
//! assert_eq!(to_telegraph("这", Table::CN), Some(6638));
//!
//! // Search both tables (Traditional first, then Simplified)
//! assert_eq!(to_telegraph("一", Table::Both), Some(1));
//!
//! // Format as the conventional 4-digit code — no heap allocation
//! assert_eq!(to_telegraph_str("一", Table::Both).unwrap(), "0001");
//! ```
//!
//! The standard conversion traits are also supported via [`TelegraphCode`]:
//!
//! ```rust
//! use chinese_telegraph::TelegraphCode;
//!
//! let code: TelegraphCode = '一'.try_into()?;
//! let num: usize = code.into();
//! assert_eq!(num, 1);
//! assert_eq!(code.to_code_str(), "0001");
//! # Ok::<(), chinese_telegraph::NoTelegraphCode>(())
//! ```
//!
//! The input must be exactly one character; longer strings return `None`.
//! To convert a sentence, iterate over its characters:
//!
//! ```rust
//! use chinese_telegraph::{Table, TelegraphCode};
//!
//! let codes: Vec<_> = "電報"
//!     .chars()
//!     .filter_map(|c| TelegraphCode::lookup(c, Table::TW))
//!     .map(|code| code.to_code_str())
//!     .collect();
//! assert_eq!(codes, ["7193", "1032"]);
//! ```
//!
//! # Feature flags
//!
//! All features are enabled by default:
//!
//! - `telegraph-code` — the [`TelegraphCode`] type with its standard
//!   conversion traits, and the [`NoTelegraphCode`] error.
//! - `code-str` — the [`CodeStr`] inline four-digit string type and
//!   [`to_telegraph_str`].
//! - `std` — adds [`to_telegraph_string`] for formatting codes as 4-digit
//!   [`String`](std::string::String)s, and (with `telegraph-code`)
//!   implements `std::error::Error` for [`NoTelegraphCode`].
//!
//! The core lookup, [`to_telegraph`], works without any features: the crate
//! is `#![no_std]` and allocation-free at its core, with heap-free
//! four-digit formatting available via `code-str`. For `no_std`
//! environments, disable `std` (or all default features) with
//! `default-features = false`.

/// Simplified Chinese character lookup table.
// Telegraph codes are canonically written as 4-digit numbers, so the tables
// keep leading zeros for readability (Rust parses these as decimal).
#[allow(clippy::zero_prefixed_literal)]
mod cn;
/// Traditional Chinese character lookup table.
#[allow(clippy::zero_prefixed_literal)]
mod tw;

/// Selects which code table(s) a lookup searches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Table {
    /// Search the Traditional Chinese ([`TW`](Table::TW)) table first, then
    /// fall back to the Simplified Chinese ([`CN`](Table::CN)) table.
    Both,
    /// Search only the Traditional Chinese (Taiwan) table.
    TW,
    /// Search only the Simplified Chinese table.
    CN,
}

/// Converts a Chinese character to its telegraph code.
///
/// `character` must be a string slice containing exactly one Chinese
/// character; `table` selects which code table(s) to search.
///
/// Returns `Some(code)` if the character is found. Telegraph codes are
/// conventionally written as four digits with leading zeros (e.g. `1` is
/// `0001`); use [`to_telegraph_string`] to get that form directly.
///
/// Returns `None` if the character is not in the selected table(s), or if
/// the input is empty or contains more than one character.
///
/// # Examples
///
/// ```rust
/// use chinese_telegraph::{to_telegraph, Table};
///
/// // Traditional Chinese character
/// assert_eq!(to_telegraph("這", Table::TW), Some(6638));
///
/// // Simplified Chinese character
/// assert_eq!(to_telegraph("这", Table::CN), Some(6638));
///
/// // Character found in both tables
/// assert_eq!(to_telegraph("一", Table::Both), Some(1));
///
/// // Unknown character
/// assert_eq!(to_telegraph("🦀", Table::Both), None);
///
/// // Multiple characters
/// assert_eq!(to_telegraph("這是", Table::Both), None);
/// ```
pub fn to_telegraph(character: &str, table: Table) -> Option<usize> {
    match table {
        Table::Both => tw::TW_TABLE
            .get(character)
            .or_else(|| cn::CN_TABLE.get(character))
            .copied(),
        Table::TW => tw::TW_TABLE.get(character).copied(),
        Table::CN => cn::CN_TABLE.get(character).copied(),
    }
}
/// The error returned when a conversion to [`TelegraphCode`] fails.
///
/// Produced by the [`TryFrom`] implementations on [`TelegraphCode`] when the
/// character is not found in the searched table(s), or when a string input
/// is not exactly one character.
#[cfg(feature = "telegraph-code")]
#[cfg_attr(docsrs, doc(cfg(feature = "telegraph-code")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoTelegraphCode;

#[cfg(feature = "telegraph-code")]
impl core::fmt::Display for NoTelegraphCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("no Chinese telegraph code for the given character")
    }
}

#[cfg(all(feature = "std", feature = "telegraph-code"))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for NoTelegraphCode {}

/// A Chinese telegraph code, usable with the standard conversion traits.
///
/// This is the trait-based counterpart to [`to_telegraph`]: converting a
/// [`char`] or `&str` with [`TryInto`] searches both tables
/// ([`Table::Both`]), and the resulting code converts [`Into`] a plain
/// [`usize`]. Use [`TelegraphCode::lookup`] to search a specific table.
///
/// The [`Display`](core::fmt::Display) implementation formats the code in
/// its conventional four-digit form with leading zeros.
///
/// # Examples
///
/// ```rust
/// use chinese_telegraph::TelegraphCode;
///
/// let code: TelegraphCode = '一'.try_into()?;
///
/// // `.into()` recovers the plain number
/// let num: usize = code.into();
/// assert_eq!(num, 1);
///
/// // Display formats the conventional 4-digit code
/// assert_eq!(code.to_string(), "0001");
///
/// // Unknown characters fail to convert
/// assert!(TelegraphCode::try_from('🦀').is_err());
/// # Ok::<(), chinese_telegraph::NoTelegraphCode>(())
/// ```
#[cfg(feature = "telegraph-code")]
#[cfg_attr(docsrs, doc(cfg(feature = "telegraph-code")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TelegraphCode(usize);

#[cfg(feature = "telegraph-code")]
impl TelegraphCode {
    /// Looks up `character` in the selected `table`.
    ///
    /// This is the [`TelegraphCode`] equivalent of [`to_telegraph`], for
    /// when the lookup should search a specific table rather than
    /// [`Table::Both`] (which the [`TryFrom`] conversions use).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chinese_telegraph::{Table, TelegraphCode};
    ///
    /// let code = TelegraphCode::lookup('這', Table::TW).unwrap();
    /// assert_eq!(usize::from(code), 6638);
    ///
    /// assert_eq!(TelegraphCode::lookup('这', Table::TW), None);
    /// ```
    pub fn lookup(character: char, table: Table) -> Option<Self> {
        let mut buf = [0u8; 4];
        to_telegraph(character.encode_utf8(&mut buf), table).map(Self)
    }

    /// Returns the code as a plain number (e.g. `1` for the code written
    /// `0001`).
    pub const fn code(self) -> usize {
        self.0
    }

    /// Formats the code in its conventional four-digit form, without heap
    /// allocation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chinese_telegraph::TelegraphCode;
    ///
    /// let code = TelegraphCode::try_from('一')?;
    /// assert_eq!(code.to_code_str(), "0001");
    /// # Ok::<(), chinese_telegraph::NoTelegraphCode>(())
    /// ```
    #[cfg(feature = "code-str")]
    #[cfg_attr(docsrs, doc(cfg(feature = "code-str")))]
    pub const fn to_code_str(self) -> CodeStr {
        CodeStr::new(self.0)
    }
}

/// The four-digit form of a telegraph code (e.g. `"0001"`), stored inline
/// without heap allocation.
///
/// Returned by [`to_telegraph_str`] and [`TelegraphCode::to_code_str`]. It
/// holds the code's four ASCII digits, including leading zeros, and can be
/// used wherever a [`&str`](str) is expected via [`as_str`](CodeStr::as_str),
/// [`Deref`](core::ops::Deref), or [`AsRef<str>`](AsRef).
///
/// # Examples
///
/// ```rust
/// use chinese_telegraph::{to_telegraph_str, Table};
///
/// let code_str = to_telegraph_str("一", Table::Both).unwrap();
/// assert_eq!(code_str, "0001");
/// assert_eq!(code_str.as_str().len(), 4);
/// ```
#[cfg(feature = "code-str")]
#[cfg_attr(docsrs, doc(cfg(feature = "code-str")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeStr([u8; 4]);

#[cfg(feature = "code-str")]
impl CodeStr {
    /// Formats `code` as four ASCII digits. `code` must be at most `9999`,
    /// which every table entry is.
    const fn new(code: usize) -> Self {
        Self([
            b'0' + (code / 1000 % 10) as u8,
            b'0' + (code / 100 % 10) as u8,
            b'0' + (code / 10 % 10) as u8,
            b'0' + (code % 10) as u8,
        ])
    }

    /// Returns the four-digit code as a string slice.
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("telegraph code digits are ASCII")
    }
}

#[cfg(feature = "code-str")]
impl core::ops::Deref for CodeStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(feature = "code-str")]
impl AsRef<str> for CodeStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "code-str")]
impl PartialEq<&str> for CodeStr {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[cfg(feature = "code-str")]
impl PartialEq<str> for CodeStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

#[cfg(feature = "code-str")]
impl core::fmt::Display for CodeStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "telegraph-code")]
impl core::fmt::Display for TelegraphCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

#[cfg(feature = "telegraph-code")]
impl TryFrom<char> for TelegraphCode {
    type Error = NoTelegraphCode;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        Self::lookup(character, Table::Both).ok_or(NoTelegraphCode)
    }
}

#[cfg(feature = "telegraph-code")]
impl TryFrom<&str> for TelegraphCode {
    type Error = NoTelegraphCode;

    fn try_from(character: &str) -> Result<Self, Self::Error> {
        to_telegraph(character, Table::Both)
            .map(Self)
            .ok_or(NoTelegraphCode)
    }
}

#[cfg(feature = "telegraph-code")]
impl From<TelegraphCode> for usize {
    fn from(code: TelegraphCode) -> Self {
        code.0
    }
}

#[cfg(feature = "std")]
extern crate std;

/// Converts a Chinese character to its telegraph code, formatted as the
/// conventional four-digit code with leading zeros.
///
/// This is the formatting counterpart of [`to_telegraph`]. The returned
/// [`CodeStr`] stores the four ASCII digits inline — no heap allocation —
/// and dereferences to [`&str`](str). It returns `None` in the same cases
/// as [`to_telegraph`]: an unknown character, or input that is not exactly
/// one character.
///
/// # Examples
///
/// ```rust
/// use chinese_telegraph::{to_telegraph_str, Table};
///
/// assert_eq!(to_telegraph_str("一", Table::Both).unwrap(), "0001");
/// assert_eq!(to_telegraph_str("這", Table::TW).unwrap(), "6638");
/// assert!(to_telegraph_str("🦀", Table::Both).is_none());
/// ```
#[cfg(feature = "code-str")]
#[cfg_attr(docsrs, doc(cfg(feature = "code-str")))]
pub fn to_telegraph_str(character: &str, table: Table) -> Option<CodeStr> {
    to_telegraph(character, table).map(CodeStr::new)
}

/// Converts a Chinese character to its telegraph code, formatted as the
/// conventional four-digit [`String`](std::string::String) with leading
/// zeros.
///
/// This is only available when the `std` feature is enabled; prefer
/// [`to_telegraph_str`], which returns the same four digits without heap
/// allocation and works in `no_std` environments. It returns `None` in the
/// same cases as [`to_telegraph`]: an unknown character, or input that is
/// not exactly one character.
///
/// # Examples
///
/// ```rust
/// use chinese_telegraph::{to_telegraph_string, Table};
///
/// assert_eq!(to_telegraph_string("一", Table::Both), Some("0001".to_string()));
/// assert_eq!(to_telegraph_string("這", Table::TW), Some("6638".to_string()));
/// assert_eq!(to_telegraph_string("🦀", Table::Both), None);
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn to_telegraph_string(character: &str, table: Table) -> Option<std::string::String> {
    to_telegraph(character, table).map(|num| std::format!("{:04}", num))
}

#[cfg(test)]
mod tests {
    use crate::to_telegraph;
    #[cfg(feature = "code-str")]
    use crate::to_telegraph_str;

    #[test]
    fn it_can_look_up_a_tw_character() {
        let result = to_telegraph("這", crate::Table::TW);
        assert_eq!(result, Some(6638));
    }

    #[test]
    fn it_can_look_up_a_cn_character() {
        let result = to_telegraph("这", crate::Table::CN);
        assert_eq!(result, Some(6638));
    }

    #[test]
    fn it_can_look_up_a_character_in_both_tables() {
        let result = to_telegraph("一", crate::Table::Both);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn it_returns_none_for_unknown_characters() {
        let result = to_telegraph("🦀", crate::Table::Both);
        assert_eq!(result, None);
    }

    #[test]
    fn it_returns_none_for_more_than_one_character() {
        let result = to_telegraph("這是", crate::Table::Both);
        assert_eq!(result, None);
    }

    #[cfg(feature = "telegraph-code")]
    #[test]
    fn it_converts_a_char_with_try_into() {
        let code: crate::TelegraphCode = '這'.try_into().unwrap();
        assert_eq!(code.code(), 6638);
    }

    #[cfg(feature = "telegraph-code")]
    #[test]
    fn it_converts_a_str_with_try_into() {
        let code: crate::TelegraphCode = "这".try_into().unwrap();
        assert_eq!(code.code(), 6638);
    }

    #[cfg(feature = "telegraph-code")]
    #[test]
    fn it_converts_a_code_into_usize() {
        let code = crate::TelegraphCode::lookup('一', crate::Table::Both).unwrap();
        let num: usize = code.into();
        assert_eq!(num, 1);
    }

    #[cfg(feature = "telegraph-code")]
    #[test]
    fn it_fails_to_convert_unknown_characters() {
        assert_eq!(
            crate::TelegraphCode::try_from('🦀'),
            Err(crate::NoTelegraphCode)
        );
        assert_eq!(
            crate::TelegraphCode::try_from("這是"),
            Err(crate::NoTelegraphCode)
        );
    }

    #[cfg(feature = "telegraph-code")]
    #[test]
    fn it_respects_the_table_in_lookup() {
        assert_eq!(crate::TelegraphCode::lookup('这', crate::Table::TW), None);
        let code = crate::TelegraphCode::lookup('这', crate::Table::CN).unwrap();
        assert_eq!(code.code(), 6638);
    }

    #[cfg(all(feature = "std", feature = "telegraph-code"))]
    #[test]
    fn it_displays_the_code_with_leading_zeros() {
        let code = crate::TelegraphCode::lookup('一', crate::Table::Both).unwrap();
        assert_eq!(std::string::ToString::to_string(&code), "0001");
    }

    #[cfg(feature = "code-str")]
    #[test]
    fn it_formats_the_number_with_leading_zeros() {
        let result = to_telegraph_str("一", crate::Table::Both);
        assert_eq!(result.unwrap(), "0001");
    }

    #[cfg(feature = "code-str")]
    #[test]
    fn it_returns_none_from_to_telegraph_str_for_unknown_input() {
        assert!(to_telegraph_str("🦀", crate::Table::Both).is_none());
        assert!(to_telegraph_str("這是", crate::Table::Both).is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn it_formats_the_number_as_a_string() {
        let result = crate::to_telegraph_string("一", crate::Table::Both);
        assert_eq!(result, Some(std::string::ToString::to_string("0001")));
    }

    #[cfg(all(feature = "telegraph-code", feature = "code-str"))]
    #[test]
    fn it_formats_a_code_without_allocating() {
        let code = crate::TelegraphCode::lookup('這', crate::Table::TW).unwrap();
        let code_str = code.to_code_str();
        assert_eq!(code_str, "6638");
        assert_eq!(code_str.as_str(), "6638");
        assert_eq!(&*code_str, "6638");
    }
}

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Chinese Telegraph Code
//!
//! This crate provides utilities for converting Unicode Chinese characters to Chinese telegraph codes.
//!
//! Chinese telegraph codes are numerical codes used historically for transmitting Chinese text over telegraph systems.
//! This crate supports both Traditional Chinese (Taiwan) and Simplified Chinese character sets.
//!
//! ## Features
//!
//! - Convert Chinese characters to telegraph codes
//! - Support for both Traditional and Simplified Chinese
//! - `no_std` compatible (with optional `std` feature for string formatting)
//! - Fast lookups using perfect hash functions
//!
//! ## Usage
//!
//! ```rust
//! use chinese_telegraph::{to_telegraph, to_telegraph_string, Table};
//!
//! // Look up a Traditional Chinese character
//! let code = to_telegraph("這", Table::TW);
//! assert_eq!(code, Some(6638));
//!
//! // Look up a Simplified Chinese character
//! let code = to_telegraph("这", Table::CN);
//! assert_eq!(code, Some(6638));
//!
//! // Search both tables
//! let code = to_telegraph("一", Table::Both);
//! assert_eq!(code, Some(1));
//!
//! // Format as a 4-digit string (requires std feature)
//! # #[cfg(feature = "std")]
//! let formatted = to_telegraph_string("一", Table::Both);
//! # #[cfg(feature = "std")]
//! assert_eq!(formatted, Some("0001".to_string()));
//! ```

/// Simplified Chinese character lookup table.
mod cn;
/// Traditional Chinese character lookup table.
mod tw;

/// Specifies which character table(s) to use for telegraph code lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Table {
    /// Search both Traditional Chinese (TW) and Simplified Chinese (CN) tables.
    /// TW table is searched first, then CN table if no match is found.
    Both,
    /// Search only the Traditional Chinese (Taiwan) character table.
    TW,
    /// Search only the Simplified Chinese character table.
    CN,
}

/// Converts a Chinese character to its telegraph code.
///
/// # Arguments
///
/// * `character` - A string slice containing exactly one Chinese character
/// * `table` - Which character table(s) to search
///
/// # Returns
///
/// Returns `Some(code)` if the character is found in the specified table(s),
/// or `None` if the character is not found or if the input contains more than one character.
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
#[cfg(feature = "std")]
extern crate std;

/// Converts a Chinese character to its telegraph code formatted as a 4-digit string.
///
/// This function is only available when the `std` feature is enabled.
///
/// # Arguments
///
/// * `character` - A string slice containing exactly one Chinese character
/// * `table` - Which character table(s) to search
///
/// # Returns
///
/// Returns `Some(formatted_code)` if the character is found, where the code is
/// formatted as a 4-digit string with leading zeros, or `None` if the character
/// is not found or if the input contains more than one character.
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
    use crate::{to_telegraph, to_telegraph_string};

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

    #[test]
    fn it_formats_the_number_with_leading_zeros() {
        let result = to_telegraph_string("一", crate::Table::Both);
        assert_eq!(result, Some(std::string::ToString::to_string("0001")));
    }
}

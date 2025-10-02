#![no_std]

mod cn;
mod tw;

pub enum Table {
    Both,
    TW,
    CN,
}

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

#[cfg(feature = "std")]
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
        let result = to_telegraph("𠜎", crate::Table::Both);
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

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

#[cfg(test)]
mod tests {
    use crate::to_telegraph;

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
}

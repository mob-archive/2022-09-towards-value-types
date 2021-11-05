#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BoardValue {
    value: u8,
}

impl BoardValue {
    pub const fn new(value: u8) -> BoardValue {
        Self { value }
    }
}

impl Default for BoardValue {
    fn default() -> Self {
        BoardValue::new(2)
    }
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;

    #[test]
    fn it_should_init_default() {
        assert_eq!(BoardValue::default(), BoardValue { value: 2 });
    }

    #[test]
    fn it_should_create_given_value() {
        assert_eq!(BoardValue::new(4), BoardValue { value: 4 });
    }

}

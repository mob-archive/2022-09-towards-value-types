#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BoardValue {
    value: u32,
}

impl BoardValue {
    pub const fn new(value: u32) -> BoardValue {
        Self { value }
    }

    pub fn get_value(self) -> u32{
        self.value
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

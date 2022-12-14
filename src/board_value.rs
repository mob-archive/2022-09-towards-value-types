use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
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

    pub fn duplicate(self) -> BoardValue {
        BoardValue::new(self.value * 2)
    }
}

impl Default for BoardValue {
    fn default() -> Self {
        BoardValue::new(2)
    }
}

impl fmt::Debug for BoardValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;

    #[test]
    fn it_should_init_default() {
        assert_eq!(BoardValue::default(),BoardValue { value: 2 });
    }

    #[test]
    fn it_should_create_given_value() {
        assert_eq!(BoardValue::new(4), BoardValue { value: 4 });
    }

    #[test]
    fn it_should_duplicate_the_value() {
        // Possibility for Rust QuickCheck: https://github.com/BurntSushi/quickcheck
        assert_eq!(BoardValue::new(4).duplicate(), BoardValue { value: 8 });
        assert_eq!(BoardValue::new(8).duplicate(), BoardValue { value: 16 });
    }

}

pub struct Board {
    value:bool
}

impl Board {

    pub fn new() -> Self {
        return Self {
            value:false
        };
    }

    pub fn has_value(&self, _row: u8, _column: u8) -> bool {
        return self.value;
    }

    pub fn set_value(&mut self, _row: u8, _column: u8, _value: u8) {
        self.value = true;
    }
}

#[cfg(test)]
mod tests {

    use crate::Board;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_initializes_an_empty_baord() {
        let b = Board::new();
        assert_eq!(b.has_value(0, 0), false);
    }

    #[test]
    fn it_sets_a_value() {
        // Arrange
        let mut b = Board::new();

        // Act
        b.set_value(0,0,2);

        // Assert
        assert_eq!(b.has_value(0, 0), true);

    }

}

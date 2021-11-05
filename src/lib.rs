pub struct Board {
    value: bool,
}

impl Board {
    pub fn new() -> Self {
        return Self { value: false };
    }

    pub fn has_value(&self, _coordinate: Coordinate) -> bool {
        return self.value;
    }

    pub fn set_value(&mut self, _coordinate: Coordinate, _value: u8) {
        self.value = true;
    }
}

pub struct Coordinate {
    pub row: u8,
    pub column: u8,
}

#[cfg(test)]
mod tests {

    use crate::Board;
    use crate::Coordinate;

    #[test]
    fn it_initializes_an_empty_baord() {
        let b = Board::new();
        assert_eq!(b.has_value(Coordinate { row: 0, column: 0 }), false);
    }

    #[test]
    fn it_sets_a_value() {
        // Arrange
        let mut b = Board::new();

        // Act
        b.set_value(Coordinate { row: 0, column: 0 }, 2);

        // Assert
        assert_eq!(b.has_value(Coordinate { row: 0, column: 0 }), true);
    }
}

use std::collections::HashMap;

pub struct Board {
    cells: HashMap<Coordinate, bool>,
}

impl Board {
    pub fn new() -> Self {
        Self { cells: HashMap::new() }
    }

    pub fn has_value(&self, coordinate: Coordinate) -> bool {
        *self.cells.get(&coordinate).unwrap_or(&false)
    }

    pub fn set_value(&mut self, coordinate: Coordinate, _value: u8) {
        self.cells.insert(coordinate, true);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub row: u8,
    pub column: u8,
}

#[cfg(test)]
mod tests {

    use crate::Board;
    use crate::Coordinate;

    const FIRST_COORDINATE: Coordinate = Coordinate { row: 0, column: 0 };
    const SECOND_COORDINATE: Coordinate = Coordinate { row: 1, column: 0 };

    #[test]
    fn it_initializes_an_empty_baord() {
        let b = Board::default();
        assert_eq!(b.has_value(FIRST_COORDINATE), false);
    }

    #[test]
    fn it_sets_a_value() {
        // Arrange
        let mut b = Board::new();

        // Act
        b.set_value(FIRST_COORDINATE, 2);

        // Assert
        assert_eq!(b.has_value(FIRST_COORDINATE), true);
    }

    #[test]
    fn it_holds_two_values() {
        let mut b = Board::default();
        b.set_value(FIRST_COORDINATE, 2);
        b.set_value(SECOND_COORDINATE, 2);

        assert_eq!(b.has_value(FIRST_COORDINATE), true);
        assert_eq!(b.has_value(SECOND_COORDINATE), true);
    }

    #[test]
    fn it_sets_a_specific_cell() {
        // Arrange
        let mut b = Board::new();

        // Act
        b.set_value(FIRST_COORDINATE, 2);

        // Assert
        assert_eq!(b.has_value(SECOND_COORDINATE), false);
    }


}

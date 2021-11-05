mod coordinate;
use crate::coordinate::Coordinate;
mod board_value;
use crate::board_value::BoardValue;
use std::collections::HashMap;

pub struct Board {
    cells: HashMap<Coordinate, BoardValue>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    pub fn has_value(&self, coordinate: Coordinate) -> bool {
        self.cells.get(&coordinate).is_some()
    }

    pub fn set_value(&mut self, coordinate: Coordinate, value: BoardValue) {
        self.cells.insert(coordinate, value);
    }

    pub fn get_value(&self, coordinate: Coordinate) -> BoardValue {
        *self.cells.get(&coordinate).unwrap()
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::Board;
    use crate::BoardValue;
    use crate::Coordinate;

    const FIRST_COORDINATE: Coordinate = Coordinate { row: 0, column: 0 };
    const SECOND_COORDINATE: Coordinate = Coordinate { row: 1, column: 0 };
    const VALUE_4: BoardValue = BoardValue::new(4);

    fn create_board_with_first_coordinate(value: BoardValue) -> Board {
        let mut b = Board::default();
        b.set_value(FIRST_COORDINATE, value);
        b
    }

    #[test]
    fn it_initializes_an_empty_baord() {
        let b = Board::default();
        assert_eq!(b.has_value(FIRST_COORDINATE), false);
    }

    #[test]
    fn it_sets_a_value() {
        let b = create_board_with_first_coordinate(BoardValue::default());
        assert_eq!(b.has_value(FIRST_COORDINATE), true);
    }

    #[test]
    fn it_holds_two_values() {
        let mut b = create_board_with_first_coordinate(BoardValue::default());
        b.set_value(SECOND_COORDINATE, BoardValue::default());

        assert_eq!(b.has_value(FIRST_COORDINATE), true);
        assert_eq!(b.has_value(SECOND_COORDINATE), true);
    }

    #[test]
    fn it_sets_a_specific_cell() {
        let b = create_board_with_first_coordinate(BoardValue::default());
        assert_eq!(b.has_value(SECOND_COORDINATE), false);
    }

    #[test]
    fn it_should_consider_coordinate_content() {
        let b = create_board_with_first_coordinate(VALUE_4);
        assert_eq!(b.has_value(Coordinate { row: 0, column: 0 }), true);
    }

    #[test]
    fn it_should_return_the_value() {
        let b = create_board_with_first_coordinate(VALUE_4);
        assert_eq!(b.get_value(FIRST_COORDINATE), VALUE_4);
    }

    #[test]
    fn it_should_return_the_values_4_and_2() {
        let mut b = create_board_with_first_coordinate(VALUE_4);
        b.set_value(SECOND_COORDINATE, BoardValue::default());

        assert_eq!(b.get_value(FIRST_COORDINATE), VALUE_4);
        assert_eq!(b.get_value(SECOND_COORDINATE), BoardValue::default());
    }

    #[test]
    #[should_panic]
    fn it_should_panic_if_asked_for_a_non_existing_value() {
        let b = Board::default();

        // Act/Assert should panic
        b.get_value(SECOND_COORDINATE);
    }
}

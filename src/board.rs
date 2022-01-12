use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;
use std::fmt;
use std::collections::HashMap;

pub type Row = [BoardValue; 4];
pub type FieldRepresentation = [Row; 4];

#[derive(Eq, PartialEq)]
pub struct Board {
    cells: HashMap<Coordinate, BoardValue>,
    size: u8,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get_representation())
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            size: 4,
        }
    }

    pub fn from_field(field: FieldRepresentation) -> Self {
        const VALUE_0: BoardValue = BoardValue::new(0);
        let mut b = Board::default();
        for row in 0..b.size {
            for column in 0..b.size {
                let value = field[row as usize][column as usize];
                if value != VALUE_0 {
                    let coordinate = Coordinate { row, column };
                    b.set_value(coordinate, value);
                }
            }
        }
        b
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

    pub fn delete_value(&mut self, coordinate: Coordinate) {
        self.cells.remove(&coordinate);
    }

    pub fn get_representation(&self) -> FieldRepresentation {
        const V: BoardValue = BoardValue::new(0);
        let mut result: FieldRepresentation =
            [[V, V, V, V], [V, V, V, V], [V, V, V, V], [V, V, V, V]];

        for row in 0..self.size {
            for column in 0..self.size {
                let coordinate = Coordinate { row, column };
                if self.has_value(coordinate) {
                    result[row as usize][column as usize] = self.get_value(coordinate);
                }
            }
        }
        result
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::board::Board;
    use crate::board::BoardValue;
    use crate::board::Coordinate;
    use crate::board::FieldRepresentation;

    const FIRST_COORDINATE: Coordinate = Coordinate { row: 0, column: 0 };
    const SECOND_COORDINATE: Coordinate = Coordinate { row: 1, column: 0 };
    const V: BoardValue = BoardValue::new(0);
    const VALUE_2: BoardValue = BoardValue::new(2);
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

        const V: BoardValue = BoardValue::new(0);
        const VALUE_2: BoardValue = BoardValue::new(2);
        assert_eq!(
            b.get_representation(),
            [[VALUE_2, V, V, V], [V, V, V, V], [V, V, V, V], [V, V, V, V]]
        )
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

    #[test]
    fn it_should_convert_to_external_representation() {
        let b = create_board_with_first_coordinate(VALUE_4);

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation =
            [[VALUE_4, V, V, V], [V, V, V, V], [V, V, V, V], [V, V, V, V]];
        assert_eq!(field, expected_field);
    }

    #[test]
    fn it_should_convert_to_external_representation_with_value_two() {
        let b = create_board_with_first_coordinate(BoardValue::new(2));

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation = [
            [BoardValue::new(2), V, V, V],
            [V, V, V, V],
            [V, V, V, V],
            [V, V, V, V],
        ];
        assert_eq!(field, expected_field);
    }

    #[test]
    fn it_should_convert_to_representation_second_coordinate() {
        let mut b = Board::default();
        b.set_value(SECOND_COORDINATE, VALUE_4);

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation =
            [[V, V, V, V], [VALUE_4, V, V, V], [V, V, V, V], [V, V, V, V]];
        assert_eq!(field, expected_field);
    }

    #[test]
    fn it_should_convert_to_representation_with_two_identical_values() {
        let mut b = Board::default();
        b.set_value(FIRST_COORDINATE, VALUE_4);
        b.set_value(SECOND_COORDINATE, VALUE_4);

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation = [
            [VALUE_4, V, V, V],
            [VALUE_4, V, V, V],
            [V, V, V, V],
            [V, V, V, V],
        ];
        assert_eq!(field, expected_field);
    }

    #[test]
    fn it_should_convert_to_representation_with_two_values() {
        let mut b = Board::default();
        b.set_value(FIRST_COORDINATE, VALUE_2);
        b.set_value(SECOND_COORDINATE, VALUE_4);

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation = [
            [VALUE_2, V, V, V],
            [VALUE_4, V, V, V],
            [V, V, V, V],
            [V, V, V, V],
        ];
        assert_eq!(field, expected_field);
    }

    #[test]
    fn it_should_convert_to_representation_in_last_row() {
        let mut b = Board::default();
        b.set_value(Coordinate { column: 0, row: 3 }, VALUE_2);
        b.set_value(Coordinate { column: 3, row: 3 }, VALUE_4);

        let field = b.get_representation();

        let expected_field: FieldRepresentation = [
            [V, V, V, V],
            [V, V, V, V],
            [V, V, V, V],
            [VALUE_2, V, V, VALUE_4],
        ];
        assert_eq!(field, expected_field);
    }

    #[test]
    fn it_should_delete_value() {
        let mut b = create_board_with_first_coordinate(VALUE_4);

        b.delete_value(FIRST_COORDINATE);

        assert_eq!(b.has_value(FIRST_COORDINATE), false);
    }

    #[test]
    fn it_should_create_a_board_from_representation() {
        let mut expected_board = Board::default();
        expected_board.set_value(Coordinate { column: 0, row: 3 }, VALUE_2);
        expected_board.set_value(Coordinate { column: 3, row: 3 }, VALUE_4);

        let board = Board::from_field([
            [V, V, V, V],
            [V, V, V, V],
            [V, V, V, V],
            [VALUE_2, V, V, VALUE_4],
        ]);

        assert_eq!(board, expected_board);
    }
}

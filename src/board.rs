use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;
use std::collections::HashMap;

pub type FieldRepresentation = [[BoardValue; 4]; 4];

#[derive(Debug, Eq, PartialEq)]
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

    pub fn get_representation(&self) -> FieldRepresentation {
        const V: BoardValue = BoardValue::new(0);
        let mut result: FieldRepresentation =
            [[V, V, V, V], [V, V, V, V], [V, V, V, V], [V, V, V, V]];

        for row in 0..4 {
            for column in 0..4 {
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
        assert_eq!(expected_field, field);
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
        assert_eq!(expected_field, field);
    }

    #[test]
    fn it_should_convert_to_representation_second_coordinate() {
        let mut b = Board::default();
        b.set_value(SECOND_COORDINATE, VALUE_4);

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation =
            [[V, V, V, V], [VALUE_4, V, V, V], [V, V, V, V], [V, V, V, V]];
        assert_eq!(expected_field, field);
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
        assert_eq!(expected_field, field);
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
        assert_eq!(expected_field, field);
    }

    #[test]
    fn it_should_convert_to_representation_in_last_row() {
        let mut b = Board::default();
        b.set_value(Coordinate { column: 0, row: 3 }, VALUE_2);
        b.set_value(Coordinate { column: 3, row: 3 }, VALUE_4);

        let field = b.get_representation();

        const V: BoardValue = BoardValue::new(0);
        let expected_field: FieldRepresentation = [
            [V, V, V, V],
            [V, V, V, V],
            [V, V, V, V],
            [VALUE_2, V, V, VALUE_4],
        ];
        assert_eq!(expected_field, field);
    }
}

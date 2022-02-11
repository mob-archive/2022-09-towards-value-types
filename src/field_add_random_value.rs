use crate::board::Field;
use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;
use crate::random::RandomNumber;

pub fn add_value(
    field: Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> Field {
    let mut new_field = field;
    let new_board_value = get_new_board_value(random_number_value);
    let coordinate = get_coordinate(field, random_number_position);
    new_field[coordinate.row as usize][coordinate.column as usize] = new_board_value;
    new_field
}

fn get_coordinate(field: Field, random_number: RandomNumber) -> Coordinate {
    let vec: Vec<Coordinate> = get_coordinates_of_empty_cells(field);
    let coordinate_index = get_index_for_coordinate(random_number, vec.len());
    vec[coordinate_index]
}

fn get_index_for_coordinate(random_number: RandomNumber, free_coordinate_count: usize) -> usize {
    // There is no 100% even distribution!
    // Through the rounding, the last and first coordinate have a lower chance to be selected
    // since they only have one half of the rounding.
    let number_for_coordinate = (free_coordinate_count as f32 * random_number).round();
    let index_for_cooridnate = (number_for_coordinate - 1.0) as usize;
    index_for_cooridnate
}

fn get_coordinates_of_empty_cells(field: Field) -> Vec<Coordinate> {
    const X: BoardValue = BoardValue::new(0);
    let mut vec: Vec<Coordinate> = Vec::new();
    for row in 0..4 as u8 {
        for column in 0..4 as u8 {
            if field[row as usize][column as usize] == X {
                vec.push(Coordinate { row, column })
            }
        }
    }
    vec
}

fn get_new_board_value(random_number: RandomNumber) -> BoardValue {
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);

    // Asumption: the four appears roughly in 10% of the cases
    let new_number: BoardValue;
    if random_number < 0.9 {
        new_number = TWO;
    } else {
        new_number = FOUR;
    }
    new_number
}

#[cfg(test)]
mod tests {

    use crate::field_add_random_value::*;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);
    const EMPTY_FIELD: Field = [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
    const NON_EMPTY_FIELD: Field = [[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
    const ONLY_ONE_EMPTY_CELL_FIELD: Field = [
        [TWO, TWO, TWO, TWO],
        [TWO, TWO, TWO, TWO],
        [TWO, TWO, TWO, TWO],
        [TWO, TWO, TWO, X],
    ];
    const FULL_FIELD: Field = [
        [TWO, TWO, TWO, TWO],
        [TWO, TWO, TWO, TWO],
        [TWO, TWO, TWO, TWO],
        [TWO, TWO, TWO, TWO],
    ];
    const FAKE_RANDOM_NUMBER_VALUE_ZERO: RandomNumber = 0.0;
    const FAKE_RANDOM_NUMBER_POSITION_ZERO: RandomNumber = 0.0;

    #[cfg(test)]
    mod given_random_number_value_zero {
        use crate::field_add_random_value::tests::*;
        #[test]
        fn it_should_return_field_with_two_in_the_first_empty_cell_for_empty_field() {
            let new_field = add_value(
                EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            let expected_field = [[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }

        #[test]
        fn it_should_return_field_with_two_in_the_first_empty_cell_for_non_empty_field() {
            let new_field = add_value(
                NON_EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            let expected_field = [[TWO, TWO, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }

        #[test]
        fn it_should_return_field_with_two_in_the_first_empty_cell_for_only_one_empty_cell_field() {
            let new_field = add_value(
                ONLY_ONE_EMPTY_CELL_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            assert_eq!(new_field, FULL_FIELD);
        }
    }

    #[cfg(test)]
    mod given_random_number_value_zero_point_one {
        use crate::field_add_random_value::tests::*;
        const FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_ONE: RandomNumber = 0.1;
        #[test]
        fn it_should_return_field_with_two_in_the_first_empty_cell_for_empty_field() {
            let new_field = add_value(
                EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_ONE,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            let expected_field = [[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }

        #[test]
        fn it_should_return_field_with_two_in_the_first_empty_cell_for_non_empty_field() {
            let new_field = add_value(
                NON_EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_ONE,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            let expected_field = [[TWO, TWO, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }
    }
    #[cfg(test)]
    mod given_random_number_value_zero_point_eight {
        use crate::field_add_random_value::tests::*;
        const FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_EIGHT: RandomNumber = 0.8;
        #[test]
        fn it_should_return_field_with_two_in_the_first_empty_cell_for_empty_field() {
            let new_field = add_value(
                EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_EIGHT,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            let expected_field = [[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }
    }

    #[cfg(test)]
    mod given_random_number_value_zero_point_nine {
        use crate::field_add_random_value::tests::*;
        const FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_NINE: RandomNumber = 0.9;
        #[test]
        fn it_should_return_field_with_four_in_the_first_empty_cell_for_empty_field() {
            let new_field = add_value(
                EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO_POINT_NINE,
                FAKE_RANDOM_NUMBER_POSITION_ZERO,
            );
            let expected_field = [[FOUR, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }
    }

    #[cfg(test)]
    mod given_random_number_position_zero_point_nine_nine {
        use crate::field_add_random_value::tests::*;
        const FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_NINE_NINE: RandomNumber = 0.99;
        #[test]
        fn it_should_return_field_with_two_in_the_last_empty_cell_for_empty_field() {
            let new_field = add_value(
                EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_NINE_NINE,
            );
            let expected_field = [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, TWO]];
            assert_eq!(new_field, expected_field);
        }
        #[test]
        fn it_should_return_field_with_two_in_the_last_empty_cell_for_non_empty_field() {
            let new_field = add_value(
                NON_EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_NINE_NINE,
            );
            let expected_field = [[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, TWO]];
            assert_eq!(new_field, expected_field);
        }
        #[test]
        fn it_should_return_field_with_two_in_the_last_empty_cell_for_last_position_filled_field() {
            const LAST_POSITION_FILLED_FIELD: Field =
                [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, TWO]];
            let new_field = add_value(
                LAST_POSITION_FILLED_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_NINE_NINE,
            );
            let expected_field = [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, TWO, TWO]];
            assert_eq!(new_field, expected_field);
        }
    }

    #[cfg(test)]
    mod given_random_number_position_zero_point_five_four {
        use crate::field_add_random_value::tests::*;
        const FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_FIVE: RandomNumber = 0.54;
        #[test]
        fn it_should_return_field_with_two_in_the_nineth_empty_cell_for_empty_field() {
            let new_field = add_value(
                EMPTY_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_FIVE,
            );
            let expected_field = [[X, X, X, X], [X, X, X, X], [TWO, X, X, X], [X, X, X, X]];
            assert_eq!(new_field, expected_field);
        }
        #[test]
        fn it_should_return_field_completly_filled_for_only_one_empty_cell_field() {
            let new_field = add_value(
                ONLY_ONE_EMPTY_CELL_FIELD,
                FAKE_RANDOM_NUMBER_VALUE_ZERO,
                FAKE_RANDOM_NUMBER_POSITION_ZERO_POINT_FIVE,
            );
            assert_eq!(new_field, FULL_FIELD);
        }
    }
}

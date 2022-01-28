use crate::board::Field;
use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;
use crate::random::RandomNumber;

pub fn add_value(
    field: Field,
    random_number_value: RandomNumber,
    _random_number_position: RandomNumber,
) -> Field {
    let new_number: BoardValue = get_new_number(random_number_value);
    let vec: Vec<Coordinate> = get_coordinates_of_empty_cells(field);
    let mut new_field: Field = field;
    let first_coordinate: Coordinate = vec[0];
    new_field[first_coordinate.row as usize][first_coordinate.column as usize] = new_number;
    new_field
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

fn get_new_number(random_number_value: RandomNumber) -> BoardValue {
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);

    let new_number: BoardValue;
    if random_number_value < 0.9 {
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
    const FAKE_RANDOM_NUMBER_POSITION_ZERO: RandomNumber = 0.0;

    #[cfg(test)]
    mod given_random_number_value_zero {
        use crate::field_add_random_value::tests::*;
        const FAKE_RANDOM_NUMBER_VALUE_ZERO: RandomNumber = 0.0;
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
}

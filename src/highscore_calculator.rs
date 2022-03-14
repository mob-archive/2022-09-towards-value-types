use crate::board::Field;
use crate::board::Row;
use crate::board_value::BoardValue;

type Highscore = u64;

pub fn calculate_added_points(_from: Field, to: Field) -> Highscore {
    // Assumption: Random field is not added yet
    // list of all fields in _from
    // list of all fields in to
    // determine disappeard fields in _from to to
    // Added theses values toghether
    let mut sum: Highscore = 0;

    if to[0][0] == BoardValue::new(4) || to[0][0] == BoardValue::new(8) {
        sum = sum + to[0][0].get_value() as Highscore;
    }
    if to[1][0] == BoardValue::new(4) || to[1][0] == BoardValue::new(8) {
        sum = sum + to[1][0].get_value() as Highscore;
    }
    sum
}

pub fn calculate_added_points_per_row_left(from: Row, to: Row) -> Highscore {
    let mut sum: Highscore = 0;
    for merged_value in get_merged_values(from, to).iter() {
        sum += merged_value.get_value() as Highscore
    }
    sum
}

fn get_merged_values(from: Row, to: Row) -> Vec<BoardValue> {
    let from_vector = get_vector_without_zeros_sorted_descending(from);
    let to_vector = get_vector_without_zeros_sorted_descending(to);

    let mut from_iterator = from_vector.iter();

    let mut merged_values: Vec<BoardValue> = Vec::new();
    for current_to in to_vector.iter() {
        let current_from = from_iterator.next();
        if current_from.unwrap() == current_to {
            continue;
        }
        from_iterator.next();
        merged_values.push(*current_to);
    }
    merged_values
}

fn get_vector_without_zeros(row: Row) -> Vec<BoardValue> {
    let mut result_vector: Vec<BoardValue> = Vec::new();
    for index in 0..4 {
        if row[index] != BoardValue::new(0) {
            result_vector.push(row[index])
        }
    }
    result_vector
}

fn get_vector_without_zeros_sorted_descending(row: Row) -> Vec<BoardValue> {
    let mut values = get_vector_without_zeros(row);
    values.sort_by(|a, b| b.cmp(a));
    values
}

#[cfg(test)]
mod tests {
    use crate::highscore_calculator::*;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);
    const EIGHT: BoardValue = BoardValue::new(8);
    const EMPTY_FIELD: Field = [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];

    #[cfg(test)]
    mod calculate_added_points_per_row_left {
        use crate::highscore_calculator::tests::*;
        const EMPTY_ROW: Row = [X, X, X, X];
        const ROW_WITH_TWO_LEFT: Row = [TWO, X, X, X];
        #[test]
        fn it_should_return_zero_if_rows_are_equal() {
            assert_eq!(calculate_added_points_per_row_left(EMPTY_ROW, EMPTY_ROW), 0);
        }

        #[test]
        fn it_should_return_zero_if_non_empty_rows_are_equal() {
            assert_eq!(
                calculate_added_points_per_row_left(ROW_WITH_TWO_LEFT, ROW_WITH_TWO_LEFT),
                0
            );
        }

        #[test]
        fn it_should_return_four_if_two_twos_are_merged() {
            assert_eq!(
                calculate_added_points_per_row_left([TWO, TWO, X, X], [FOUR, X, X, X]),
                4
            );
        }

        #[test]
        fn it_should_return_eight_if_four_twos_are_merged() {
            assert_eq!(
                calculate_added_points_per_row_left([TWO, TWO, TWO, TWO], [FOUR, FOUR, X, X]),
                8
            );
        }

        #[test]
        fn it_should_return_zero_if_nothing_was_merged() {
            assert_eq!(
                calculate_added_points_per_row_left([TWO, FOUR, X, X], [TWO, FOUR, X, X]),
                0
            );
        }

        #[test]
        fn it_should_return_four_last_two_twos_are_merged() {
            assert_eq!(
                calculate_added_points_per_row_left(
                    [EIGHT, FOUR, TWO, TWO],
                    [EIGHT, FOUR, FOUR, X]
                ),
                4
            );
        }
        #[test]
        fn it_should_return_zero_if_two_was_just_moved() {
            assert_eq!(
                calculate_added_points_per_row_left([X, X, X, TWO], [TWO, X, X, X]),
                0
            );
        }
        #[test]
        fn it_should_return_zero_if_four_was_just_moved() {
            assert_eq!(
                calculate_added_points_per_row_left([X, X, TWO, FOUR], [TWO, FOUR, X, X]),
                0
            );
        }

        #[test]
        fn it_should_return_zero_if_eight_was_just_moved() {
            assert_eq!(
                calculate_added_points_per_row_left([X, TWO, FOUR, EIGHT], [TWO, FOUR, EIGHT, X]),
                0
            );
        }

        #[test]
        fn it_should_return_four_if_two_twos_are_moved_and_merged() {
            assert_eq!(
                calculate_added_points_per_row_left([X, TWO, X, TWO], [FOUR, X, X, X]),
                4
            );
        }

        #[test]
        fn it_should_return_four_if_two_twos_are_merged_and_one_is_moved() {
            assert_eq!(
                calculate_added_points_per_row_left([X, TWO, TWO, TWO], [FOUR, TWO, X, X]),
                4
            );
        }

        #[test]
        fn it_should_return_four_if_two_twos_are_merged_and_one_four_is_moved() {
            assert_eq!(
                calculate_added_points_per_row_left([X, TWO, TWO, FOUR], [FOUR, FOUR, X, X]),
                4
            );
        }

        #[test]
        fn it_should_return_four_if_two_fours_are_merged_and_two_twos_are_moved() {
            assert_eq!(
                calculate_added_points_per_row_left([TWO, FOUR, FOUR, TWO], [TWO, EIGHT, TWO, X]),
                8
            );
        }

        #[test]
        fn it_should_return_four_if_two_fours_and_two_twos_are_merged() {
            assert_eq!(
                calculate_added_points_per_row_left([FOUR, FOUR, TWO, TWO], [EIGHT, FOUR, X, X]),
                12
            );
        }
    }
    #[test]
    fn it_should_return_zero_if_fields_are_equal() {
        assert_eq!(calculate_added_points(EMPTY_FIELD, EMPTY_FIELD), 0);
    }
    #[test]
    fn it_should_return_four_if_two_twos_are_merged() {
        #[rustfmt::skip]
        assert_eq!(calculate_added_points(
            [
              [TWO, TWO, X, X], 
              [X, X, X, X],
              [X, X, X, X],
              [X, X, X, X]
            ], [
              [FOUR, X, X, X], 
              [X, X, X, X],
              [X, X, X, X],
              [X, X, X, X]
            ]
        ), 4);
    }
    #[test]
    fn it_should_return_eight_if_four_twos_are_merged() {
        #[rustfmt::skip]
        assert_eq!(calculate_added_points(
            [
              [TWO, TWO, X, X], 
              [TWO, TWO, X, X],
              [X, X, X, X],
              [X, X, X, X]
            ], [
              [FOUR, X, X, X], 
              [FOUR, X, X, X],
              [X, X, X, X],
              [X, X, X, X]
            ]
        ), 8);
    }
}

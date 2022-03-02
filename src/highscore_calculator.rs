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
    if has_same_values(from, to) {
        return 0;
    }
    let mut sum: Highscore = 0;
    let mut index = 0;
    if from[index] != to[index] {
        sum += to[index].get_value() as Highscore
    }
    index += 1;
    if from[index] != to[index] {
        sum += to[index].get_value() as Highscore
    }
    index += 1;
    if from[index] != to[index] {
        sum += to[index].get_value() as Highscore
    }
    sum
}

pub fn has_same_values(from: Row, to: Row) -> bool {
    let from_vector: Vec<BoardValue> = get_vector_without_zeros(from);
    let to_vector: Vec<BoardValue> = get_vector_without_zeros(to);
    from_vector == to_vector
}

fn get_vector_without_zeros(row: Row) -> Vec<BoardValue>{
    let mut result_vector: Vec<BoardValue> = Vec::new();
    for index in 0..4 {
        if row[index] != BoardValue::new(0) {
            result_vector.push(row[index])
        }
    }
    result_vector
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
    mod has_same_values {
        use crate::highscore_calculator::tests::*;
        #[test]
        fn it_should_return_true_if_tripplet_is_moved_from_right_to_left() {
            assert_eq!(
                has_same_values([X, TWO, FOUR, EIGHT], [TWO, FOUR, EIGHT, X]),
                true
            );
            assert_eq!(
                has_same_values([TWO, FOUR, X, EIGHT], [TWO, FOUR, EIGHT, X]),
                true
            );
            assert_eq!(
                has_same_values([TWO, X, X, EIGHT], [TWO, EIGHT, X, X]),
                true
            );
        }
        #[test]
        fn it_should_return_false_if_two_twos_were_merged() {
            assert_eq!(
                has_same_values([TWO, TWO, X, X], [FOUR, X, X, X]),
                false
            );
        }

        #[test]
        fn it_should_return_false_if_two_fours_were_merged() {
            assert_eq!(
                has_same_values([FOUR, FOUR, X, X], [EIGHT, X, X, X]),
                false
            );
        }
    }
    #[cfg(test)]
    mod calculate_added_points_per_row_left {
        use crate::highscore_calculator::tests::*;
        const EMPTY_ROW: Row = [X, X, X, X];
        const ROW_WITH_TWO_LEFT: Row = [X, X, X, X];
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

        // #[test]
        // fn it_should_return_four_if_two_twos_are_merged_and_one_four_is_moved() {
        //     assert_eq!(
        //         calculate_added_points_per_row_left([X, TWO, TWO, FOUR], [FOUR, FOUR, X, X]),
        //         4
        //     );
        // }
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

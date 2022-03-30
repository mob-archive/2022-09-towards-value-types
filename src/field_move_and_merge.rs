use crate::field::Field;
use crate::field::Row;
use crate::board_value::BoardValue;
use crate::field_rotate::*;

pub fn move_and_merge_left(field: Field) -> Field {
    [
        move_and_merge_row_left(field[0]),
        move_and_merge_row_left(field[1]),
        move_and_merge_row_left(field[2]),
        move_and_merge_row_left(field[3]),
    ]
}

pub fn move_and_merge_up(field: Field) -> Field {
    rotate_clockwise(move_and_merge_left(rotate_counterclockwise(field)))
}

pub fn move_and_merge_down(field: Field) -> Field {
    rotate_counterclockwise(move_and_merge_left(rotate_clockwise(field)))
}

pub fn move_and_merge_right(field: Field) -> Field {
    rotate_clockwise(move_and_merge_up(rotate_counterclockwise(field)))
}

fn move_and_merge_row_left(row: Row) -> Row {
    let mut result: Row = row; // Implicit clone

    for index in 0..3 {
        result = pull(result, index);
        result = merge(result, index);
    }
    result
}

fn pull(row: Row, index: usize) -> Row {
    let mut result = row;
    let next_value = find_next_value(result, index + 1);
    if result[index] == BoardValue::new(0) {
        result[index] = result[next_value];
        result[next_value] = BoardValue::new(0);
    }
    result
}

fn merge(row: Row, index: usize) -> Row {
    let mut result = row;
    let next_value = find_next_value(result, index + 1);
    if result[index] == result[next_value] {
        result[index] = result[index].duplicate();
        result[next_value] = BoardValue::new(0);
        result = shift_values(result, index + 1);
    }
    result
}

fn find_next_value(row: Row, start_index: usize) -> usize {
    let mut result: usize = start_index;
    for (index, cell) in row.iter().enumerate().skip(start_index) {
        if *cell != BoardValue::new(0) {
            result = index;
            break;
        }
    }
    result
}

fn shift_values(row: Row, start_index: usize) -> Row {
    let mut result: Row = row;
    for index in start_index..=2 {
        let source_index = index + 1;
        result[index] = result[source_index];
        result[source_index] = BoardValue::new(0);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::field::tests::*;
    use crate::field_move_and_merge::Field;
    const ALL_VALUES_LEFT: Field = [
        [TWO, X, X, X],
        [FOUR, X, X, X],
        [EIGHT, X, X, X],
        [SIXTEEN, X, X, X],
    ];
    const ALL_VALUES_RIGHT: Field = [
        [X, X, X, TWO],
        [X, X, X, FOUR],
        [X, X, X, EIGHT],
        [X, X, X, SIXTEEN],
    ];
    const ALL_VALUES_DOWN: Field = [
        [X, X, X, X],
        [X, X, X, X],
        [X, X, X, X],
        [SIXTEEN, EIGHT, FOUR, TWO],
    ];
    const ALL_VALUES_UP: Field = [
        [SIXTEEN, EIGHT, FOUR, TWO],
        [X, X, X, X],
        [X, X, X, X],
        [X, X, X, X],
    ];

    #[cfg(test)]
    mod tests_move_and_merge_down {
        use crate::field_move_and_merge::move_and_merge_down;
        use crate::field_move_and_merge::tests::*;

        #[test]
        fn it_should_do_nothing_on_an_empty_field() {
            assert_eq!(move_and_merge_down(EMPTY_FIELD), EMPTY_FIELD);
        }

        #[test]
        fn it_should_move_values_in_all_columns_downwards() {
            assert_eq!(move_and_merge_down(ALL_VALUES_UP), ALL_VALUES_DOWN);
        }

        #[test]
        fn it_should_merge_values_in_all_columns_downwards() {
            assert_eq!(
                move_and_merge_down([
                    [EIGHT, EIGHT, FOUR, TWO],
                    [EIGHT, EIGHT, FOUR, TWO],
                    [X, X, X, X],
                    [X, X, X, X]
                ]),
                [
                    [X, X, X, X],
                    [X, X, X, X],
                    [X, X, X, X],
                    [SIXTEEN, SIXTEEN, EIGHT, FOUR]
                ]
            );
        }
    }

    #[cfg(test)]
    mod tests_move_and_merge_up {
        use crate::field_move_and_merge::move_and_merge_up;
        use crate::field_move_and_merge::tests::*;

        #[test]
        fn it_should_do_nothing_on_an_empty_field() {
            assert_eq!(move_and_merge_up(EMPTY_FIELD), EMPTY_FIELD);
        }

        #[test]
        fn it_should_move_values_in_all_columns_upwards() {
            assert_eq!(move_and_merge_up(ALL_VALUES_DOWN), ALL_VALUES_UP);
        }

        #[test]
        fn it_should_merge_values_in_all_columns_upwards() {
            assert_eq!(
                move_and_merge_up([
                    [X, X, X, X],
                    [X, X, X, X],
                    [EIGHT, EIGHT, FOUR, TWO],
                    [EIGHT, EIGHT, FOUR, TWO]
                ]),
                [
                    [SIXTEEN, SIXTEEN, EIGHT, FOUR],
                    [X, X, X, X],
                    [X, X, X, X],
                    [X, X, X, X]
                ]
            );
        }
    }

    #[cfg(test)]
    mod tests_move_and_merge_right {
        use crate::field_move_and_merge::move_and_merge_right;
        use crate::field_move_and_merge::tests::*;

        #[test]
        fn it_should_do_nothing_on_an_empty_field() {
            assert_eq!(move_and_merge_right(EMPTY_FIELD), EMPTY_FIELD);
        }

        #[test]
        fn it_should_move_values_in_all_rows_to_the_right() {
            assert_eq!(move_and_merge_right(ALL_VALUES_LEFT), ALL_VALUES_RIGHT);
        }

        #[test]
        fn it_should_merge_values_in_all_rows_to_the_left() {
            assert_eq!(
                move_and_merge_right([
                    [TWO, TWO, X, X],
                    [FOUR, FOUR, X, X],
                    [EIGHT, EIGHT, X, X],
                    [EIGHT, EIGHT, X, X]
                ]),
                [
                    [X, X, X, FOUR],
                    [X, X, X, EIGHT],
                    [X, X, X, SIXTEEN],
                    [X, X, X, SIXTEEN]
                ]
            );
        }
    }

    #[cfg(test)]
    mod tests_move_and_merge_left {
        use crate::field_move_and_merge::move_and_merge_left;
        use crate::field_move_and_merge::tests::*;

        #[test]
        fn it_should_do_nothing_on_an_empty_field() {
            assert_eq!(move_and_merge_left(EMPTY_FIELD), EMPTY_FIELD);
        }

        #[test]
        fn it_should_move_values_in_all_rows_to_the_left() {
            assert_eq!(move_and_merge_left(ALL_VALUES_RIGHT), ALL_VALUES_LEFT);
        }

        #[test]
        fn it_should_merge_values_in_all_rows_to_the_left() {
            assert_eq!(
                move_and_merge_left([
                    [X, X, TWO, TWO],
                    [X, X, FOUR, FOUR],
                    [X, X, EIGHT, EIGHT],
                    [X, X, EIGHT, EIGHT]
                ]),
                [
                    [FOUR, X, X, X],
                    [EIGHT, X, X, X],
                    [SIXTEEN, X, X, X],
                    [SIXTEEN, X, X, X]
                ]
            );
        }
    }

    #[cfg(test)]
    mod tests_move_and_merge_row_left {

        use crate::field_move_and_merge::move_and_merge_row_left;
        use crate::field_move_and_merge::tests::*;

        #[test]
        fn it_should_do_nothing_on_an_empty_row() {
            assert_eq!(move_and_merge_row_left([X, X, X, X]), [X, X, X, X]);
        }

        // only move
        #[test]
        fn it_should_move_one_value_from_last_to_first_spot() {
            assert_eq!(move_and_merge_row_left([X, X, X, TWO]), [TWO, X, X, X]);
        }

        #[test]
        fn it_should_move_one_value_from_third_to_first_spot() {
            assert_eq!(move_and_merge_row_left([X, X, TWO, X]), [TWO, X, X, X]);
        }

        #[test]
        fn it_should_not_move_value_in_first_spot() {
            assert_eq!(move_and_merge_row_left([TWO, X, X, X]), [TWO, X, X, X]);
        }
        #[test]
        fn it_should_move_one_value_from_second_to_first_spot() {
            assert_eq!(move_and_merge_row_left([X, TWO, X, X]), [TWO, X, X, X]);
        }

        #[test]
        fn it_should_stack_different_values() {
            assert_eq!(
                move_and_merge_row_left([TWO, X, FOUR, X]),
                [TWO, FOUR, X, X]
            );
        }

        #[test]
        fn it_should_move_stacked_values() {
            assert_eq!(
                move_and_merge_row_left([X, TWO, FOUR, X]),
                [TWO, FOUR, X, X]
            );
        }

        #[test]
        fn it_should_not_move_when_all_spots_have_different_values() {
            assert_eq!(
                move_and_merge_row_left([TWO, FOUR, EIGHT, SIXTEEN]),
                [TWO, FOUR, EIGHT, SIXTEEN]
            );
        }

        // only merge
        #[test]
        fn it_should_merge_same_values() {
            assert_eq!(move_and_merge_row_left([TWO, TWO, X, X]), [FOUR, X, X, X]);
        }

        #[test]
        fn it_should_merge_two_values_with_four() {
            assert_eq!(
                move_and_merge_row_left([FOUR, FOUR, X, X]),
                [EIGHT, X, X, X]
            );
        }

        #[test]
        fn it_should_merge_same_values_but_apart() {
            assert_eq!(move_and_merge_row_left([TWO, X, X, TWO]), [FOUR, X, X, X]);
        }

        #[test]
        fn it_should_merge_same_values_but_apart_not_in_first_spot() {
            assert_eq!(move_and_merge_row_left([X, TWO, X, TWO]), [FOUR, X, X, X]);
        }

        #[test]
        fn it_should_merge_two_same_values_and_leave_the_third_same_value() {
            assert_eq!(
                move_and_merge_row_left([TWO, TWO, TWO, X]),
                [FOUR, TWO, X, X]
            );
        }

        #[test]
        fn it_should_merge_two_same_values_and_leave_the_third_same_value_first_spot_empty() {
            assert_eq!(
                move_and_merge_row_left([X, TWO, TWO, TWO]),
                [FOUR, TWO, X, X]
            );
        }

        #[test]
        fn it_should_merge_both_twins_of_same_values() {
            assert_eq!(
                move_and_merge_row_left([TWO, TWO, TWO, TWO]),
                [FOUR, FOUR, X, X]
            );
        }

        // move and merge
        #[test]
        fn it_should_merge_and_move_two_same_values() {
            assert_eq!(
                move_and_merge_row_left([FOUR, X, TWO, TWO]),
                [FOUR, FOUR, X, X]
            );
        }

        #[test]
        fn it_should_merge_and_move_two_same_values_empty_in_between() {
            assert_eq!(
                move_and_merge_row_left([FOUR, TWO, X, TWO]),
                [FOUR, FOUR, X, X]
            );
        }

        #[test]
        fn it_should_merge_and_move_two_same_values_first_spot_empty() {
            assert_eq!(
                move_and_merge_row_left([X, FOUR, TWO, TWO]),
                [FOUR, FOUR, X, X]
            );
        }

        #[test]
        fn it_should_merge_and_move_two_same_values_in_between_other_values() {
            assert_eq!(
                move_and_merge_row_left([FOUR, TWO, TWO, FOUR]),
                [FOUR, FOUR, FOUR, X]
            );
        }
    }
}

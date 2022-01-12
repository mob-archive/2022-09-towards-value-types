use crate::board::FieldRepresentation;
use crate::board::Row;
use crate::board_value::BoardValue;

pub fn move_and_merge_left(field: FieldRepresentation) -> FieldRepresentation {
    [
        move_and_merge(field[0]),
        move_and_merge(field[1]),
        move_and_merge(field[2]),
        move_and_merge(field[3]),
    ]
}

fn move_and_merge(row: Row) -> Row {
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
mod tests_move_and_merge_left {
    use crate::game_mover::move_and_merge_left;
    use crate::game_mover::BoardValue;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);
    const EIGHT: BoardValue = BoardValue::new(8);
    const SIXTEEN: BoardValue = BoardValue::new(16);

    #[test]
    fn it_should_do_nothing_on_an_empty_field() {
        assert_eq!(
            move_and_merge_left([[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]]),
            [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]]
        );
    }

    #[test]
    fn it_should_move_values_in_all_rows_to_the_left() {
        assert_eq!(
            move_and_merge_left([
                [X, X, X, TWO],
                [X, X, X, FOUR],
                [X, X, X, EIGHT],
                [X, X, X, SIXTEEN]
            ]),
            [
                [TWO, X, X, X],
                [FOUR, X, X, X],
                [EIGHT, X, X, X],
                [SIXTEEN, X, X, X]
            ]
        );
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
mod tests_move_and_merge {

    use crate::game_mover::move_and_merge;
    use crate::game_mover::BoardValue;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);
    const EIGHT: BoardValue = BoardValue::new(8);
    const SIXTEEN: BoardValue = BoardValue::new(16);

    #[test]
    fn it_should_do_nothing_on_an_empty_row() {
        assert_eq!(move_and_merge([X, X, X, X]), [X, X, X, X]);
    }

    // only move
    #[test]
    fn it_should_move_one_value_from_last_to_first_spot() {
        assert_eq!(move_and_merge([X, X, X, TWO]), [TWO, X, X, X]);
    }

    #[test]
    fn it_should_move_one_value_from_third_to_first_spot() {
        assert_eq!(move_and_merge([X, X, TWO, X]), [TWO, X, X, X]);
    }

    #[test]
    fn it_should_not_move_value_in_first_spot() {
        assert_eq!(move_and_merge([TWO, X, X, X]), [TWO, X, X, X]);
    }
    #[test]
    fn it_should_move_one_value_from_second_to_first_spot() {
        assert_eq!(move_and_merge([X, TWO, X, X]), [TWO, X, X, X]);
    }

    #[test]
    fn it_should_stack_different_values() {
        assert_eq!(move_and_merge([TWO, X, FOUR, X]), [TWO, FOUR, X, X]);
    }

    #[test]
    fn it_should_move_stacked_values() {
        assert_eq!(move_and_merge([X, TWO, FOUR, X]), [TWO, FOUR, X, X]);
    }

    #[test]
    fn it_should_not_move_when_all_spots_have_different_values() {
        assert_eq!(
            move_and_merge([TWO, FOUR, EIGHT, SIXTEEN]),
            [TWO, FOUR, EIGHT, SIXTEEN]
        );
    }

    // only merge
    #[test]
    fn it_should_merge_same_values() {
        assert_eq!(move_and_merge([TWO, TWO, X, X]), [FOUR, X, X, X]);
    }

    #[test]
    fn it_should_merge_two_values_with_four() {
        assert_eq!(move_and_merge([FOUR, FOUR, X, X]), [EIGHT, X, X, X]);
    }

    #[test]
    fn it_should_merge_same_values_but_apart() {
        assert_eq!(move_and_merge([TWO, X, X, TWO]), [FOUR, X, X, X]);
    }

    #[test]
    fn it_should_merge_same_values_but_apart_not_in_first_spot() {
        assert_eq!(move_and_merge([X, TWO, X, TWO]), [FOUR, X, X, X]);
    }

    #[test]
    fn it_should_merge_two_same_values_and_leave_the_third_same_value() {
        assert_eq!(move_and_merge([TWO, TWO, TWO, X]), [FOUR, TWO, X, X]);
    }

    #[test]
    fn it_should_merge_two_same_values_and_leave_the_third_same_value_first_spot_empty() {
        assert_eq!(move_and_merge([X, TWO, TWO, TWO]), [FOUR, TWO, X, X]);
    }

    #[test]
    fn it_should_merge_both_twins_of_same_values() {
        assert_eq!(move_and_merge([TWO, TWO, TWO, TWO]), [FOUR, FOUR, X, X]);
    }

    // move and merge
    #[test]
    fn it_should_merge_and_move_two_same_values() {
        assert_eq!(move_and_merge([FOUR, X, TWO, TWO]), [FOUR, FOUR, X, X]);
    }

    #[test]
    fn it_should_merge_and_move_two_same_values_empty_in_between() {
        assert_eq!(move_and_merge([FOUR, TWO, X, TWO]), [FOUR, FOUR, X, X]);
    }

    #[test]
    fn it_should_merge_and_move_two_same_values_first_spot_empty() {
        assert_eq!(move_and_merge([X, FOUR, TWO, TWO]), [FOUR, FOUR, X, X]);
    }

    #[test]
    fn it_should_merge_and_move_two_same_values_in_between_other_values() {
        assert_eq!(
            move_and_merge([FOUR, TWO, TWO, FOUR]),
            [FOUR, FOUR, FOUR, X]
        );
    }
}

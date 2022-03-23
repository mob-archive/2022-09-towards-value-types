use crate::field::Field;
use crate::field::Row;

pub fn rotate_counterclockwise(field: Field) -> Field {
    [
        extract_column_reversed(field, 3),
        extract_column_reversed(field, 2),
        extract_column_reversed(field, 1),
        extract_column_reversed(field, 0),
    ]
}

fn extract_column_reversed(field: Field, column_index: usize) -> Row {
    [
        field[0][column_index],
        field[1][column_index],
        field[2][column_index],
        field[3][column_index],
    ]
}

pub fn rotate_clockwise(field: Field) -> Field {
    [
        extract_column(field, 0),
        extract_column(field, 1),
        extract_column(field, 2),
        extract_column(field, 3),
    ]
}

fn extract_column(field: Field, index: usize) -> Row {
    [
        field[3][index],
        field[2][index],
        field[1][index],
        field[0][index],
    ]
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;
    use crate::field_rotate::*;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const EMPTY_FIELD: Field = [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];

    #[cfg(test)]
    mod tests_rotate_clockwise {
        use crate::field_rotate::tests::*;

        #[test]
        fn it_should_rotate_empty_field() {
            assert_eq!(rotate_clockwise(EMPTY_FIELD), EMPTY_FIELD);
        }
        #[test]
        fn it_should_rotate_top_left_value_to_top_right_value() {
            assert_eq!(
                rotate_clockwise([[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]]),
                [[X, X, X, TWO], [X, X, X, X], [X, X, X, X], [X, X, X, X]]
            );
        }
    }

    #[cfg(test)]
    mod tests_rotate_counterclockwise {
        use crate::field_rotate::tests::*;
        #[test]
        fn it_should_rotate_empty_field() {
            assert_eq!(rotate_counterclockwise(EMPTY_FIELD), EMPTY_FIELD);
        }
        #[test]
        fn it_should_rotate_top_left_value_to_top_right_value() {
            assert_eq!(
                rotate_counterclockwise([[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]]),
                [[X, X, X, X], [X, X, X, X], [X, X, X, X], [TWO, X, X, X]]
            );
        }
    }
}

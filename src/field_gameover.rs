use crate::board_value::BoardValue;
use crate::field::Field;

pub fn is_game_over(field: Field) -> bool {
    let tuples = get_all_cell_neighbours(field);
    for (current, neighbour) in tuples.iter() {
        if current == &BoardValue::new(0) {
            return false;
        }
        if current == neighbour {
            return false;
        }
    }
    true
}

fn get_all_cell_neighbours(field: Field) -> Vec<(BoardValue, BoardValue)> {
    let mut tuples: Vec<(BoardValue, BoardValue)> = vec![];
    for row in 0..4 {
        for column in 0..4 {
            let current = field[row][column];
            if row < 3 {
                tuples.push((current, field[row + 1][column]));
            }
            if column < 3 {
                tuples.push((current, field[row][column + 1]));
            }
        }
    }
    tuples
}

#[cfg(test)]
mod test {
    use crate::field_gameover::*;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);
    const EIGHT: BoardValue = BoardValue::new(8);

    #[test]
    fn it_should_return_all_touples_in_an_full_field() {
        const FULL_FIELD: Field = [
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
        ];
        let actual = get_all_cell_neighbours(FULL_FIELD);
        assert_eq!(24, actual.len());
    }

    #[test]
    fn it_should_return_true_if_field_is_full_of_unmergeable_numbers() {
        const FULL_FIELD: Field = [
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
        ];
        assert_eq!(true, is_game_over(FULL_FIELD));
    }

    #[test]
    fn it_should_return_false_if_field_can_be_moved_down() {
        const LAST_ROW_EMPTY: Field = [
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
            [X, X, X, X],
        ];
        assert_eq!(false, is_game_over(LAST_ROW_EMPTY));
    }

    #[test]
    fn it_should_return_false_if_field_can_be_moved_up() {
        const FIRST_ROW_EMPTY: Field = [
            [X, X, X, X],
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
        ];
        assert_eq!(false, is_game_over(FIRST_ROW_EMPTY));
    }

    #[test]
    fn it_should_return_false_if_field_can_be_moved_left() {
        const LEFT_COLUMN_EMPTY: Field = [
            [X, FOUR, TWO, FOUR],
            [X, TWO, FOUR, TWO],
            [X, FOUR, TWO, FOUR],
            [X, TWO, FOUR, TWO],
        ];
        assert_eq!(false, is_game_over(LEFT_COLUMN_EMPTY));
    }

    #[test]
    fn it_should_return_false_if_field_can_be_moved_right() {
        const RIGHT_COLUMN_EMPTY: Field = [
            [TWO, FOUR, TWO, X],
            [FOUR, TWO, FOUR, X],
            [TWO, FOUR, TWO, X],
            [FOUR, TWO, FOUR, X],
        ];
        assert_eq!(false, is_game_over(RIGHT_COLUMN_EMPTY));
    }

    #[test]
    fn it_should_return_false_if_field_can_be_merged_right() {
        const FULL_BUT_MERGEABLE: Field = [
            [TWO, FOUR, EIGHT, EIGHT], // can be merged left or right
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
        ];
        assert_eq!(false, is_game_over(FULL_BUT_MERGEABLE));
    }

    #[test]
    fn it_should_return_false_if_one_field_is_zero() {
        const ONE_FIELD_ZERO: Field = [
            [TWO, FOUR, TWO, X],
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
        ];
        assert_eq!(false, is_game_over(ONE_FIELD_ZERO));
    }

    #[test]
    fn it_should_return_false_if_field_can_be_merged_right_by_same_value_instances() {
        const EIGHT_ONE:BoardValue = BoardValue::new(8);
        const EIGHT_TWO:BoardValue = BoardValue::new(8);
        const FULL_BUT_MERGEABLE: Field = [
            [TWO, FOUR, EIGHT_ONE, EIGHT_TWO], // can be merged left or right
            [FOUR, TWO, FOUR, TWO],
            [TWO, FOUR, TWO, FOUR],
            [FOUR, TWO, FOUR, TWO],
        ];
        assert_eq!(false, is_game_over(FULL_BUT_MERGEABLE));
    }
}

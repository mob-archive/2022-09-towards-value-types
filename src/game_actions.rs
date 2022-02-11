use crate::board::Board;
use crate::board::Field;
use crate::field_add_random_value::add_value;
use crate::field_move_and_merge::*;
use crate::random::RandomNumber;

pub fn move_down(
    board: &mut Board,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) {
    move_board(
        board,
        move_and_merge_down,
        random_number_value,
        random_number_position,
    );
}

pub fn move_right(
    board: &mut Board,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) {
    move_board(
        board,
        move_and_merge_right,
        random_number_value,
        random_number_position,
    );
}

pub fn move_up(
    board: &mut Board,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) {
    move_board(
        board,
        move_and_merge_up,
        random_number_value,
        random_number_position,
    );
}

pub fn move_left(
    board: &mut Board,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) {
    move_board(
        board,
        move_and_merge_left,
        random_number_value,
        random_number_position,
    );
}

fn move_board(
    board: &mut Board,
    move_and_merge_operation: fn(Field) -> Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) {
    let old_field = board.get_representation();
    let moved = move_and_merge_operation(old_field);
    // TODO: gained_points = calculate_points(matrix, neue_matrix)
    // TODO: if something moved/merged
    let with_new_value = add_value(moved, random_number_value, random_number_position);
    board.save_representation(with_new_value);
    // TODO: return game_action_result (including gained_points)
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;
    use crate::game_actions::Board;
    use crate::game_actions::*;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);

    #[test]
    fn it_should_perform_the_move_down_action() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [TWO, TWO, X, TWO],
            [TWO, X, X, TWO],
            [X, X, X, TWO],
            [X, X, TWO, TWO]
        ]);

        move_down(&mut board, 0.0, 0.0);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [TWO, X, X, X],
            [X, X, X, X],
            [X, X, X, FOUR],
            [FOUR, TWO, TWO, FOUR]
        ]));
    }

    #[test]
    fn it_should_perform_the_move_right_action() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [TWO, TWO, X, TWO],
            [TWO, X, X, TWO],
            [X, X, X, TWO],
            [X, X, TWO, TWO]
        ]);

        move_right(&mut board, 0.0, 0.0);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [TWO, X, TWO, FOUR],
            [X, X, X, FOUR],
            [X, X, X, TWO],
            [X, X, X, FOUR]
        ]));
    }

    #[test]
    fn it_should_perform_the_move_up_action() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [TWO, TWO, X, TWO],
            [TWO, X, X, TWO],
            [X, X, X, TWO],
            [X, X, TWO, TWO]
        ]);

        move_up(&mut board, 0.0, 0.0);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [FOUR, TWO , TWO, FOUR],
            [TWO, X, X, FOUR],
            [X, X, X, X],
            [X, X, X, X]
        ]));
    }

    #[test]
    fn it_should_perform_the_move_left_action() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [TWO, TWO, X, TWO],
            [TWO, X, X, TWO],
            [X, X, X, TWO],
            [X, X, TWO, TWO]
        ]);

        move_left(&mut board, 0.99, 0.99);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [FOUR, TWO, X, X],
            [FOUR, X, X, X],
            [TWO, X, X, X],
            [FOUR, X, X, FOUR]
        ]));
    }
}

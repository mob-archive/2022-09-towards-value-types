use crate::board::Board;
use crate::board::Field;
use crate::field_move_and_merge::*;

pub fn move_down(board: &mut Board) {
    move_board(board, move_and_merge_down);
}

pub fn move_right(board: &mut Board) {
    move_board(board, move_and_merge_right);
}

pub fn move_up(board: &mut Board) {
    move_board(board, move_and_merge_up);
}

pub fn move_left(board: &mut Board) {
    move_board(board, move_and_merge_left);
}

fn move_board(board: &mut Board, move_and_merge_operation:fn(Field) -> Field) {
    let old_field = board.get_representation();
    let new_field = move_and_merge_operation(old_field);
    board.save_representation(new_field);
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;
    use crate::game_actions::*;
    use crate::game_actions::Board;

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

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
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

        move_right(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, TWO, FOUR],
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

        move_up(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [FOUR, TWO , TWO, FOUR],
            [X, X, X, FOUR],
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

        move_left(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [FOUR, TWO, X, X],
            [FOUR, X, X, X],
            [TWO, X, X, X],
            [FOUR, X, X, X]
        ]));
    }
}

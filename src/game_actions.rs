use crate::board::Board;
use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;

pub fn move_down(board: &mut Board) {
    if board.has_value(Coordinate { row: 2, column: 0 }) {
        board.set_value(Coordinate { row: 3, column: 0 }, BoardValue::default());
    }
}

#[cfg(test)]
mod tests {

    use crate::game_actions::move_down;
    use crate::game_actions::Board;
    use crate::game_actions::BoardValue;
    use crate::game_actions::Coordinate;

    #[test]
    fn it_should_do_nothing_on_empty_board() {
        let mut board = Board::default();

        move_down(&mut board);

        assert_eq!(board, Board::default());
    }

    #[test]
    fn it_should_move_one_value_down() {
        let mut board = Board::default();
        board.set_value(Coordinate { row: 2, column: 0 }, BoardValue::default());

        move_down(&mut board);

        assert_eq!(
            board.get_value(Coordinate { row: 3, column: 0 }),
            BoardValue::default()
        );
    }
}

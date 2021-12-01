use crate::board::Board;
use crate::coordinate::Coordinate;

pub fn move_down(board: &mut Board) {
    move_cell_value(
        board,
        Coordinate { row: 2, column: 0 },
        Coordinate { row: 3, column: 0 },
    );
    move_cell_value(
        board,
        Coordinate { row: 2, column: 1 },
        Coordinate { row: 3, column: 1 },
    );
}
fn move_cell_value(board: &mut Board, from: Coordinate, to: Coordinate) {
    if board.has_value(from) {
        let previous_value = board.get_value(from);
        board.delete_value(from);
        board.set_value(to, previous_value);
    }
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;
    use crate::game_actions::move_down;
    use crate::game_actions::Board;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);

    #[test]
    fn it_should_do_nothing_on_empty_board() {
        let mut board = Board::default();

        move_down(&mut board);

        assert_eq!(board, Board::default());
    }

    #[test]
    fn it_should_move_one_value_down() {
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X],
            [X, X, X, X],
        ]);

        move_down(&mut board);

        assert_eq!(
            board,
            Board::from_field([
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X]
            ])
        );
    }

    #[test]
    fn it_should_move_two_values_down() {
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, TWO, X, X],
            [X, X, X, X],
        ]);

        move_down(&mut board);

        assert_eq!(
            board,
            Board::from_field([
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, TWO, X, X]
            ])
        );
    }

    #[test]
    fn it_should_move_two_different_values_down() {
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, FOUR, X, X],
            [X, X, X, X],
        ]);

        move_down(&mut board);

        assert_eq!(
            board,
            Board::from_field([
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, FOUR, X, X]
            ])
        );
    }
}

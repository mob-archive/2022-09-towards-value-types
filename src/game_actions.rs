use crate::board::Board;
use crate::coordinate::Coordinate;

pub fn move_down(board: &mut Board) {
    for _round2 in 0..=3 {
        for round in 1..=3 {
            let row: u8 = 3 - round;
            for column in 0..4 {
                move_cell_value_down(board, Coordinate { row, column });
            }
        }
    }
}

fn move_cell_value_down(board: &mut Board, from: Coordinate) {
    let to = Coordinate {
        row: from.row + 1,
        column: from.column,
    };
    if board.has_value(from) {
        if board.has_value(to) {
            if board.get_value(to) == board.get_value(from) {
                board.set_value(to, board.get_value(from).duplicate());
                board.delete_value(from);
            }
        } else {
            board.set_value(to, board.get_value(from));
            board.delete_value(from);
        }
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
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X],
            [X, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X]
        ]));
    }

    #[test]
    fn it_should_move_two_values_down() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, TWO, X, X],
            [X, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [TWO, TWO, X, X]
        ]));
    }

    #[test]
    fn it_should_move_two_different_values_down() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, FOUR, X, X],
            [X, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [TWO, FOUR, X, X]
        ]));
    }

    #[test]
    fn it_should_move_values_down_in_all_columns() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, TWO, TWO, TWO],
            [X, X, X, X],
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [TWO, TWO, TWO, TWO]
        ]));
    }

    #[test]
    fn it_should_move_values_down_from_top_row_to_bottom_in_first_column() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [TWO, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X]
        ]));
    }

    #[test]
    fn it_should_move_two_different_values_one_down_in_the_same_column() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [TWO, X, X, X],
            [FOUR, X, X, X],
            [X, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X],
                [FOUR, X, X, X]
        ]));
    }

    #[test]
    fn it_should_move_two_different_values_down_in_the_same_column() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [TWO, X, X, X],
            [FOUR, X, X, X],
            [X, X, X, X],
            [X, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X],
            [FOUR, X, X, X]
        ]));
    }

    #[test]
    fn it_should_merge_two_equal_values_in_the_same_column() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X],
            [TWO, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [FOUR, X, X, X]
        ]));
    }

    #[test]
    fn it_should_merge_two_equal_values_in_the_same_column_but_one_row_apart() {
        #[rustfmt::skip]
        let mut board = Board::from_field([
            [X, X, X, X],
            [TWO, X, X, X],
            [X, X, X, X],
            [TWO, X, X, X]
        ]);

        move_down(&mut board);

        #[rustfmt::skip]
        assert_eq!(board, Board::from_field([
            [X, X, X, X],
            [X, X, X, X],
            [X, X, X, X],
            [FOUR, X, X, X]
        ]));
    }
}

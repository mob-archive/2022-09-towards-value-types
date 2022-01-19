use crate::board::Board;
use crate::field_move_and_merge::*;

pub fn move_down(board: &mut Board) {
    let old_field = board.get_representation();
    let new_field = move_and_merge_down(old_field);
    board.save_representation(new_field);
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

}

use crate::board::Board;
use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;
use crate::game_actions::move_down;

pub struct Game {
    board: Board,
}

type ExternalFieldRepresentation = [[u32; 4]; 4];

impl Game {
    pub fn new() -> Self {
        let mut board = Board::default();
        board.set_value(Coordinate { row: 2, column: 0 }, BoardValue::new(2));
        Self { board }
    }

    pub fn get_score(&self) -> u128 {
        0
    }

    pub fn get_field(&self) -> ExternalFieldRepresentation {
        let field_iter = self.board.get_representation().into_iter();
        // Rust Vector<->Array madness solved with help from
        // https://stackoverflow.com/a/63355120 (and comments)
        field_iter
            .map(|column| {
                column
                    .into_iter()
                    .map(|cell| cell.get_value())
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[u32; 4]>>()
            .try_into()
            .unwrap()
    }

    pub fn move_down(&mut self) {
        move_down(&mut self.board);
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::game::ExternalFieldRepresentation;
    use crate::game::Game;

    #[test]
    fn it_initializes_a_new_game_with_zero_score() {
        let game = Game::new();
        assert_eq!(game.get_score(), 0);
    }

    #[test]
    fn it_initializes_a_default_game_with_zero_score() {
        let game = Game::default();
        assert_eq!(game.get_score(), 0);
    }

    #[test]
    fn it_initializes_a_non_initial_field() {
        let game = Game::new();

        let field = game.get_field();

        let expected: ExternalFieldRepresentation;
        expected = [[0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 0, 0], [0, 0, 0, 0]];

        assert_eq!(field, expected);
    }

    #[test]
    fn it_moves_down() {
        let mut game = Game::new();

        game.move_down();
        let field = game.get_field();

        let expected: ExternalFieldRepresentation;
        expected = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 0, 0]];

        assert_eq!(field, expected);
    }
}

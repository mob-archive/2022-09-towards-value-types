use crate::board::Board;
use crate::board_value::BoardValue;
use crate::coordinate::Coordinate;
use crate::game_actions::*;

pub struct Game {
    board: Board,
}

pub type ExternalFieldRepresentation = Vec<u32>;

impl Game {
    pub fn new() -> Self {
        let mut board = Board::default();
        board.set_value(Coordinate { row: 2, column: 0 }, BoardValue::new(2));
        Self { board }
    }

    pub fn get_score(&self) -> u64 {
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
            })
            .collect::<Vec<Vec<u32>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<u32>>()
    }

    pub fn move_down(&mut self) {
        move_down(&mut self.board, 0.0, 0.0);
    }

    pub fn move_right(&mut self) {
        move_right(&mut self.board, 0.0, 0.0);
    }

    pub fn move_up(&mut self) {
        move_up(&mut self.board, 0.0, 0.0);
    }

    pub fn move_left(&mut self) {
        move_left(&mut self.board, 0.0, 0.0);
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
        expected = vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(field, expected);
    }

    #[test]
    fn it_moves_down() {
        let mut game = Game::new();

        game.move_down();

        assert_eq!(
            game.get_field(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0]
        );
    }
    #[test]
    fn it_moves_right() {
        let mut game = Game::new();

        game.move_right();

        assert_eq!(
            game.get_field(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0]
        );
    }
    #[test]
    fn it_moves_up() {
        let mut game = Game::new();

        game.move_up();

        assert_eq!(
            game.get_field(),
            vec![2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn it_moves_left() {
        let mut game = Game::new();

        game.move_left();

        assert_eq!(
            game.get_field(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}

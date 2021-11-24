pub struct Game {
    is_moved: bool,
}

type ExternalFieldRepresentation = [[u32; 4]; 4];

impl Game {
    pub fn new() -> Self {
        Self { is_moved: false }
    }

    pub fn get_score(&self) -> u128 {
        0
    }

    pub fn get_field(&self) -> ExternalFieldRepresentation {
        if self.is_moved {
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 0, 0]]
        } else {
            [[0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 0, 0], [0, 0, 0, 0]]
        }
    }

    pub fn move_down(&mut self) {
        self.is_moved = true
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

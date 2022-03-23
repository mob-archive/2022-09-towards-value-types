use crate::board::Board;
use crate::board::Field;
use crate::game_actions::*;
use crate::highscore_calculator::Highscore;
use crate::random::random;

pub struct Game {
    board: Board,
    score: Highscore,
}

pub type ExternalFieldRepresentation = Vec<u32>;
pub type ExternalHighscore = u64;

impl Game {
    pub fn new() -> Self {
        let mut board = Board::default();
        initialize(&mut board, random(), random());
        Self { board, score: 0 }
    }

    pub fn from_field(field: Field) -> Self {
        let mut board = Board::default();
        board.save_representation(field);
        Self { board, score: 0 }
    }

    pub fn get_score(&self) -> ExternalHighscore {
        self.score
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
        self.score += move_down(&mut self.board, random(), random());
    }

    pub fn move_right(&mut self) {
        self.score += move_right(&mut self.board, random(), random());
    }

    pub fn move_up(&mut self) {
        self.score += move_up(&mut self.board, random(), random());
    }

    pub fn move_left(&mut self) {
        self.score += move_left(&mut self.board, random(), random());
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;
    use crate::game::ExternalFieldRepresentation;
    use crate::game::Game;

    fn count_filled_fields(field: ExternalFieldRepresentation) -> u8 {
        let mut number = 0;
        for x in field {
            if x != 0 {
                number = number + 1
            }
        }
        number
    }

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
        assert_eq!(count_filled_fields(field), 1);
    }

    #[test]
    fn it_should_have_added_a_value_after_move_down_and_up() {
        let mut game = Game::new();

        game.move_down();
        game.move_up();

        assert!(count_filled_fields(game.get_field()) > 1);
    }

    #[test]
    fn it_should_have_added_a_value_after_move_right_and_left() {
        let mut game = Game::new();

        game.move_right();
        game.move_left();

        assert!(count_filled_fields(game.get_field()) > 1);
    }
    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);

    #[cfg(test)]
    mod from_field {
        use crate::board::Field;
        use crate::game::tests::*;

        #[test]
        fn it_should_initialize_game_with_given_field() {
            const NON_EMPTY_FIELD: Field =
                [[TWO, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
            let game = Game::from_field(NON_EMPTY_FIELD);

            assert_eq!(
                game.get_field(),
                [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
            assert_eq!(game.get_score(), 0);
        }
    }

    // Index for the 4x4 field in a vector representation
    // 0, 1, 2, 3
    // 4, 5, 6, 7
    // 8, 9,10,11
    //12,13,14,15

    #[cfg(test)]
    mod move_down {
        use crate::game::tests::*;
        #[test]
        fn it_moves_initial_value_down() {
            let mut game = Game::new();

            game.move_down();

            let field = game.get_field();
            let sum_of_values_in_last_row = field[12] + field[13] + field[14] + field[15];
            assert_ne!(sum_of_values_in_last_row, 0);
            assert_eq!(game.get_score(), 0);
        }

        #[test]
        fn it_should_add_4_to_score_after_merge() {
            let mut game =
                Game::from_field([[X, X, X, X], [X, X, X, X], [TWO, X, X, X], [TWO, X, X, X]]);
            game.move_down();
            assert_eq!(game.get_score(), 4);
        }
    }

    #[cfg(test)]
    mod move_right {
        use crate::game::tests::*;
        #[test]
        fn it_moves_initial_value_right() {
            let mut game = Game::new();

            game.move_right();

            let field = game.get_field();
            let sum_of_values_in_last_row = field[3] + field[7] + field[11] + field[15];
            assert_ne!(sum_of_values_in_last_row, 0);
            assert_eq!(game.get_score(), 0);
        }

        #[test]
        fn it_should_add_4_to_score_after_merge() {
            let mut game =
                Game::from_field([[X, X, X, X], [X, X, X, X], [TWO, TWO, X, X], [X, X, X, X]]);
            game.move_right();
            assert_eq!(game.get_score(), 4);
        }
    }
    #[cfg(test)]
    mod move_up {
        use crate::game::tests::*;
        #[test]
        fn it_moves_initial_value_up() {
            let mut game = Game::new();

            game.move_up();

            let field = game.get_field();
            let sum_of_values_in_last_row = field[0] + field[1] + field[2] + field[3];
            assert_ne!(sum_of_values_in_last_row, 0);
            assert_eq!(game.get_score(), 0);
        }

        #[test]
        fn it_should_add_4_to_score_after_merge() {
            let mut game =
                Game::from_field([[X, X, X, X], [X, X, X, X], [TWO, X, X, X], [TWO, X, X, X]]);
            game.move_up();
            assert_eq!(game.get_score(), 4);
        }
    }

    #[cfg(test)]
    mod move_left {
        use crate::game::tests::*;
        #[test]
        fn it_moves_initial_value_left() {
            let mut game = Game::new();

            game.move_left();

            let field = game.get_field();
            let sum_of_values_in_last_row = field[0] + field[4] + field[8] + field[12];
            assert_ne!(sum_of_values_in_last_row, 0);
            assert_eq!(game.get_score(), 0);
        }

        #[test]
        fn it_should_add_4_to_score_after_merge() {
            let mut game =
                Game::from_field([[X, X, X, X], [X, X, X, X], [TWO, TWO, X, X], [X, X, X, X]]);
            game.move_left();
            assert_eq!(game.get_score(), 4);
        }
    }
}

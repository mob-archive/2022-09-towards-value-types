use crate::field::Field;
use crate::field_add_random_value::add_value;
use crate::field_move_and_merge::*;
use crate::highscore_calculator::*;
use crate::random::RandomNumber;

pub fn move_field_down(
    field: Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> (Field, Highscore) {
    move_field(
        field,
        move_and_merge_down,
        calculate_added_points_down,
        random_number_value,
        random_number_position,
    )
}

pub fn move_field_right(
    field: Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> (Field, Highscore) {
    move_field(
        field,
        move_and_merge_right,
        calculate_added_points_right,
        random_number_value,
        random_number_position,
    )
}

pub fn move_field_up(
    field: Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> (Field, Highscore) {
    move_field(
        field,
        move_and_merge_up,
        calculate_added_points_up,
        random_number_value,
        random_number_position,
    )
}

pub fn move_field_left(
    field: Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> (Field, Highscore) {
    move_field(
        field,
        move_and_merge_left,
        calculate_added_points_left,
        random_number_value,
        random_number_position,
    )
}

fn move_field(
    field: Field,
    move_and_merge_operation: fn(Field) -> Field,
    highscore_calculate_function: fn(Field, Field) -> Highscore,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> (Field, Highscore) {
    let mut moved = move_and_merge_operation(field);
    let gained_points = highscore_calculate_function(field, moved);
    if field != moved {
        moved = add_value(moved, random_number_value, random_number_position);
    }
    (moved, gained_points)
}

pub fn initialize_field(
    field: Field,
    random_number_value: RandomNumber,
    random_number_position: RandomNumber,
) -> Field {
    add_value(field, random_number_value, random_number_position)
}

#[cfg(test)]
mod tests {

    use crate::board_value::BoardValue;
    use crate::game_actions::*;

    const X: BoardValue = BoardValue::new(0);
    const TWO: BoardValue = BoardValue::new(2);
    const FOUR: BoardValue = BoardValue::new(4);
    #[cfg(test)]
    mod move_field_down {
        use crate::game_actions::tests::*;
        #[test]
        fn it_should_perform_the_action() {
            #[rustfmt::skip]
            let source_field = [
                [TWO, TWO, X, TWO],
                [TWO, X, X, TWO],
                [X, X, X, TWO],
                [X, X, TWO, TWO]
            ];
            let (new_field, _) = move_field_down(source_field, 0.0, 0.0);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [TWO, X, X, X],
                [X, X, X, X],
                [X, X, X, FOUR],
                [FOUR, TWO, TWO, FOUR]
            ]);
        }

        #[test]
        fn it_should_not_add_value_if_nothing_changed() {
            #[rustfmt::skip]
            let source_field = [
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X]
            ];
            let (new_field, _) = move_field_down(source_field, 0.0, 0.0);
            #[rustfmt::skip]
            assert_eq!(new_field, [
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X]
            ]);
        }

        #[test]
        fn it_should_return_added_points() {
            #[rustfmt::skip]
            let source_field = [
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X],
                [TWO, X, X, X]
            ];
            let (_, actual_added_points) = move_field_down(source_field, 0.0, 0.0);
            assert_eq!(actual_added_points, 4);
        }
    }

    #[cfg(test)]
    mod move_field_right {
        use crate::game_actions::tests::*;
        #[test]
        fn it_should_perform_the_action() {
            #[rustfmt::skip]
            let source_field = [
                [TWO, TWO, X, TWO],
                [TWO, X, X, TWO],
                [X, X, X, TWO],
                [X, X, TWO, TWO]
            ];

            let (new_field, _) = move_field_right(source_field, 0.0, 0.0);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [TWO, X, TWO, FOUR],
                [X, X, X, FOUR],
                [X, X, X, TWO],
                [X, X, X, FOUR]
            ]);
        }

        #[test]
        fn it_should_not_add_value_if_nothing_changed() {
            #[rustfmt::skip]
            let source_field = [
                [X, X, X, TWO],
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X]
            ];

            let (new_field, _) = move_field_right(source_field, 0.0, 0.0);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [X, X, X, TWO],
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X]
            ]);
        }

        #[test]
        fn it_should_return_added_points() {
            #[rustfmt::skip]
            let source_field = [
                [X, X, TWO, TWO],
                [X, X, X, X],
                [X, TWO, TWO, X],
                [X, X, X, X]
            ];

            let (_, actual_added_points) = move_field_right(source_field, 0.0, 0.0);
            assert_eq!(actual_added_points, 8);
        }
    }

    #[cfg(test)]
    mod move_field_up {
        use crate::game_actions::tests::*;

        #[test]
        fn it_should_perform_the_action() {
            #[rustfmt::skip]
            let source_field = [
                [TWO, TWO, X, TWO],
                [TWO, X, X, TWO],
                [X, X, X, TWO],
                [X, X, TWO, TWO]
            ];

            let (new_field, _) = move_field_up(source_field, 0.0, 0.0);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [FOUR, TWO , TWO, FOUR],
                [TWO, X, X, FOUR],
                [X, X, X, X],
                [X, X, X, X]
            ]);
        }

        #[test]
        fn it_should_not_add_value_if_nothing_changed() {
            #[rustfmt::skip]
            let source_field= [
                [TWO, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X]
            ];

            let (new_field, _) = move_field_up(source_field, 0.0, 0.0);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [TWO, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X]
            ]);
        }

        #[test]
        fn it_should_return_added_points() {
            #[rustfmt::skip]
            let source_field = [
                [TWO, X, X, FOUR],
                [TWO, X, X, FOUR],
                [X, X, X, X],
                [X, X, X, X]
            ];

            let (_, actual_added_points) = move_field_up(source_field, 0.0, 0.0);

            assert_eq!(actual_added_points, 12);
        }
    }

    #[cfg(test)]
    mod move_field_left {
        use crate::game_actions::tests::*;
        #[test]
        fn it_should_perform_the_action() {
            #[rustfmt::skip]
            let source_field = [
                [TWO, TWO, X, TWO],
                [TWO, X, X, TWO],
                [X, X, X, TWO],
                [X, X, TWO, TWO]
            ];

            let (new_field, _) = move_field_left(source_field, 0.99, 0.99);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [FOUR, TWO, X, X],
                [FOUR, X, X, X],
                [TWO, X, X, X],
                [FOUR, X, X, FOUR]
            ]);
        }

        #[test]
        fn it_should_not_add_value_if_nothing_changed() {
            #[rustfmt::skip]
            let source_field= [
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X]
            ];

            let (new_field, _) = move_field_left(source_field, 0.0, 0.0);

            #[rustfmt::skip]
            assert_eq!(new_field, [
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, X, X, X]
            ]);
        }

        #[test]
        fn it_should_return_added_points() {
            #[rustfmt::skip]
            let source_field = [
                [X, FOUR, FOUR, X],
                [X, X, X, X],
                [X, X, X, X],
                [TWO, TWO, TWO, TWO]
            ];

            let (_, actual_added_points) = move_field_left(source_field, 0.0, 0.0);

            assert_eq!(actual_added_points, 16);
        }
    }

    #[cfg(test)]
    mod test_initialize {
        use crate::game_actions::tests::*;

        #[test]
        fn it_should_initialize_the_board() {
            #[rustfmt::skip]
            let source_field = [
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X]
            ];
            let new_field = initialize_field(source_field, 0.0, 0.0);
            #[rustfmt::skip]
            assert_eq!(new_field, [
                [TWO, X, X, X],
                [X, X, X, X],
                [X, X, X, X],
                [X, X, X, X]
            ]);
        }
    }
}

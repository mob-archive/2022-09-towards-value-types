use crate::board_value::BoardValue;

pub type Row = [BoardValue; 4];
pub type Field = [Row; 4];

#[cfg(test)]
pub mod tests {
    use crate::field::*;
    pub const X: BoardValue = BoardValue::new(0);
    pub const TWO: BoardValue = BoardValue::new(2);
    pub const FOUR: BoardValue = BoardValue::new(4);
    pub const EIGHT: BoardValue = BoardValue::new(8);
    pub const SIXTEEN: BoardValue = BoardValue::new(16);
    pub const EMPTY_FIELD: Field = [[X, X, X, X], [X, X, X, X], [X, X, X, X], [X, X, X, X]];
}

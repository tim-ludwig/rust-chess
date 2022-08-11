use crate::piece::Piece;
use std::str::FromStr;

pub struct Board {
    squares: [Piece; 64]
}

impl Board {
    pub fn new() -> Board {
        Board{ squares: [Piece::None; 64] }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Board, Self::Err> {
        todo!()
    }
}
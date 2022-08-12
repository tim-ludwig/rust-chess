use crate::piece::Piece;
use std::str::FromStr;

#[derive(Debug)]
pub struct Board {
    squares: [Board::Cell; 64]
}

impl Board {
    type Cell = Option<Piece>;

    pub fn new() -> Board {
        Board{ squares: [None; 64] }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Board, Self::Err> {
        todo!()
    }
}
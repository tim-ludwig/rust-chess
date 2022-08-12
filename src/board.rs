use crate::piece::Piece;
use std::str::FromStr;

#[derive(Debug)]
pub struct Board {
    cells: [Cell; 64]
}

type Cell = Option<Piece>;

impl Board {
    pub fn new() -> Board {
        Board{ cells: [None; 64] }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Board, Self::Err> {
        todo!()
    }
}
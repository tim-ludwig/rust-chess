use crate::piece::Piece;

pub struct Board {
    squares: [Piece; 64]
}

impl Board {
    pub fn new() -> Board {
        Board{ squares: [Piece::None; 64] }
    }
}
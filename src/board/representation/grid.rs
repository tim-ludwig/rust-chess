use crate::board::position::Position;
use crate::board::representation::BoardRepr;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Grid {
    cells: [Option<Piece>; 64]
}

impl Grid {
    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        self.cells[pos.idx()]
    }
}

impl BoardRepr for Grid {
    fn new() -> Self {
        Self {
            cells: [None; 64]
        }
    }

    fn put_piece(&mut self, pos: &Position, p: Option<Piece>) -> Option<Piece> {
        let captured = self.cells[pos.idx()];
        self.cells[pos.idx()] = p;
        captured
    }
}
use crate::board::position::Position;
use crate::board::representation::BoardRepr;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Grid {
    cells: [Option<Piece>; 64]
}

impl BoardRepr for Grid {
    fn new() -> Self {
        Self {
            cells: [None; 64]
        }
    }

    fn get_piece(&self, pos: &Position) -> Option<Piece> {
        self.cells[pos.idx()]
    }

    fn pos_of_piece(&self, p: &Piece) -> Option<Position> {
        for idx in 0..64 {
            if let Some(curr) = self.cells[idx] {
                if curr == *p {
                    return Some(idx.into())
                }
            }
        }

        None
    }

    fn put_piece(&mut self, pos: &Position, p: Option<Piece>) -> Option<Piece> {
        let captured = self.cells[pos.idx()];
        self.cells[pos.idx()] = p;
        captured
    }
}
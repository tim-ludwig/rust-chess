use crate::board::position::Position;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Grid {
    cells: [Option<Piece>; 64]
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [None; 64]
        }
    }

    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        self.cells[pos.idx()]
    }

    pub fn put_piece(&mut self, pos: &Position, p: Option<Piece>) -> Option<Piece> {
        let captured = self.get_piece(pos);
        self.cells[pos.idx()] = p;
        captured
    }

    pub fn remove_piece(&mut self, pos: &Position) -> Option<Piece> {
        self.put_piece(pos, None)
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) -> Option<Piece> {
        let moved = self.remove_piece(from);
        self.put_piece(to, moved)
    }
}
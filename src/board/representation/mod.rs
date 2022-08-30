pub mod grid;

use crate::board::position::Position;
use crate::piece::Piece;

pub trait BoardRepr {
    fn new() -> Self;

    fn put_piece(&mut self, pos: &Position, p: Option<Piece>) -> Option<Piece>;

    fn remove_piece(&mut self, pos: &Position) -> Option<Piece> {
        self.put_piece(pos, None)
    }

    fn move_piece(&mut self, from: &Position, to: &Position) -> Option<Piece> {
        let moved = self.remove_piece(from);
        self.put_piece(to, moved)
    }
}
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone)]
pub enum Piece {
    None,
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color)
}
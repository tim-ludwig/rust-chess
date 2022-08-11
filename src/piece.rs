pub enum Color {
    White,
    Black
}

pub enum Piece {
    None,
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color)
}
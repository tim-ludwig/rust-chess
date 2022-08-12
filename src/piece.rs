#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}
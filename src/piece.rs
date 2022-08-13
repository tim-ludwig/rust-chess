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

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}

impl Color {
    pub fn from_fen_char(c: &char) -> Color {
        if c.is_uppercase() { Color::White } else { Color::Black }
    }
}

impl PieceType {
    pub fn from_fen_char(c: &char) -> Option<PieceType> {
        Some(match c.to_ascii_lowercase() {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'p' => PieceType::Pawn,
            _ => return None
        })
    }
}

impl Piece {
    pub fn from_fen_char(fen_char: &char) -> Option<Piece> {
        let c = Color::from_fen_char(fen_char);
        let t = match PieceType::from_fen_char(fen_char) {
            None => return None,
            Some(t) => t
        };

        Some(Piece { color: c, piece_type: t })
    }
}
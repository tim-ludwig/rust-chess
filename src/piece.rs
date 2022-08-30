use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}

impl Color {
    pub fn from_fen_char(c: &char) -> Color {
        if c.is_uppercase() { Color::White } else { Color::Black }
    }

    pub fn iter() -> impl Iterator<Item=Color> {
        [Color::White, Color::Black].iter().copied()
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

    pub fn get_fen_char(&self) -> char {
        match self {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Pawn => 'p'
        }
    }

    pub fn iter() -> impl Iterator<Item=PieceType> {
        [PieceType::King, PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight, PieceType::Pawn].iter().copied()
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

    pub fn fen_char(&self) -> char {
        let c = self.piece_type.get_fen_char();

        if self.color == Color::White { c.to_ascii_uppercase() } else { c }
    }

    pub fn figurine(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::King => '♚',
                PieceType::Queen => '♛',
                PieceType::Rook => '♜',
                PieceType::Bishop => '♝',
                PieceType::Knight => '♞',
                PieceType::Pawn => '♟',
            }
            Color::Black => match self.piece_type {
                PieceType::King => '♔',
                PieceType::Queen => '♕',
                PieceType::Rook => '♖',
                PieceType::Bishop => '♗',
                PieceType::Knight => '♘',
                PieceType::Pawn => '♙',
            }
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.figurine())
    }
}
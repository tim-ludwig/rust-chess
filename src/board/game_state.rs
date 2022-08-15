use crate::piece::Piece;

#[derive(Copy, Clone, Debug)]
pub struct CastlingState {
    pub white_short: bool,
    pub white_long: bool,
    pub black_short: bool,
    pub black_long: bool,
}

impl CastlingState {
    pub fn new() -> CastlingState {
        CastlingState {
            white_short: true,
            white_long: true,
            black_short: true,
            black_long: true
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    pub castling: CastlingState,
    pub en_passant_file: Option<u8>,
    pub fifty_move_counter: u8,
    pub captured_piece: Option<Piece>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            castling: CastlingState::new(),
            en_passant_file: None,
            fifty_move_counter: 0,
            captured_piece: None
        }
    }

    pub fn next(&self) -> Self {
        GameState {
            castling: self.castling,
            en_passant_file: None,
            fifty_move_counter: self.fifty_move_counter + 1,
            captured_piece: None
        }
    }
}
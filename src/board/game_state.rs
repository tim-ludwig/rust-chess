use crate::piece::Piece;
use std::str::FromStr;
use crate::board::ParseFenError;

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
            white_short: false,
            white_long: false,
            black_short: false,
            black_long: false
        }
    }
}

impl FromStr for CastlingState {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = CastlingState::new();

        let mut count = 0;
        if s.contains('K') { state.white_short = true; count += 1; }
        if s.contains('Q') { state.white_long  = true; count += 1; }
        if s.contains('k') { state.black_short = true; count += 1; }
        if s.contains('q') { state.black_long  = true; count += 1; }

        if count == 0 && s != "-" || count > 0 && s.len() != count { return Err(ParseFenError{description:format!("invalid castling rights '{}'", s)}) }

        Ok(state)
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
pub mod position;

use crate::piece::Piece;
use position::Position;
use std::str::FromStr;

#[derive(Debug)]
pub struct Board {
    cells: [Option<Piece>; 64]
}

impl Board {
    pub fn new() -> Board {
        Board{ cells: [None; 64] }
    }

    pub fn get_piece(&self, pos: &Position) -> Option<Piece> { self.cells[pos.idx()] }

    pub fn put_piece(&mut self, pos: &Position, p: Option<Piece>) -> Option<Piece> {
        let captured = self.cells[pos.idx()];
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

#[derive(Debug)]
pub struct ParseFenError {
    description: String
}

impl FromStr for Board {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Board, ParseFenError> {
        let mut b = Board::new();
        let mut iter = s.split_whitespace();

        let fen_pos = match iter.next() {
            Some(fen_pos) => fen_pos,
            None => return Err(Self::Err{description:format!("Invalid fen string '{}': no position supplied", s)})
        };

        b.read_fen_pos(fen_pos)?;

        Ok(b)
    }
}

impl Board {
    fn read_fen_pos(&mut self, fen_pos: &str) -> Result<(), ParseFenError> {
        let mut rank: usize = 7;
        let mut file: usize = 0;

        for (idx, c) in fen_pos.chars().enumerate() {
            if c == '/' {
                if file != 8 || rank == 0 { return Err(ParseFenError{description:format!("Invalid fen string '{}': didn't expect '/' at pos {}", fen_pos, idx)}); }

                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                let offset: usize = c.to_digit(10).unwrap().try_into().unwrap();
                if offset > 8 || file + offset > 8 { return Err(ParseFenError{description:format!("Invalid fen string '{}': invalid offset {} at pos {}", fen_pos, offset, idx)}); }

                file += offset;
            } else {
                if file >= 8 { return Err(ParseFenError{description:format!("Invalid fen string '{}': position goes out of bounds at pos {}", fen_pos, idx)}) }

                let p = match Piece::from_fen_char(&c) {
                    Some(p) => p,
                    None => return Err(ParseFenError{description:format!("Invalid fen string '{}': '{}' at pos {} isn't a fen char", fen_pos, c, idx)})
                };

                self.put_piece(&(rank, file).into(), Some(p));
                file += 1;
            }
        }

        Ok(())
    }
}
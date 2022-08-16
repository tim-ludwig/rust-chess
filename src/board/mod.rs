pub mod position;
mod game_state;

use std::fmt::{Display, Formatter};
use std::ptr::write;
use crate::piece::{Color, Piece};
use position::Position;
use std::str::FromStr;
use crate::board::game_state::GameState;

#[derive(Debug)]
pub struct Board {
    cells: [Option<Piece>; 64],
    state_stack: Vec<GameState>,
    current_player: Color,
}

impl Board {
    pub fn new() -> Board {
        Board{ cells: [None; 64], state_stack: vec![GameState::new()], current_player: Color::White }
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

    fn get_state(&self) -> &GameState  {
        unsafe { self.state_stack.last().expect("game state stack should not be empty") }
    }

    fn get_state_mut(&mut self) -> &mut GameState  {
        unsafe { self.state_stack.last_mut().expect("game state stack should not be empty") }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓")?;

        for rank in (0..8).rev() {
            write!(f, "┃")?;

            for file in 0..8 {
                match self.get_piece(&Position::from(rank, file)) {
                    None => write!(f, "   "),
                    Some(p) => write!(f, " {} ", p)
                }?;

                if file != 7 { write!(f, "│")?; }
            }

            writeln!(f, "┃")?;
            if rank != 0 { writeln!(f, "┠───┼───┼───┼───┼───┼───┼───┼───┨")?; }
        }
        writeln!(f, "┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛")?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ParseFenError {
    description: String
}

impl FromStr for Board {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Board, ParseFenError> {
        let mut b = Board::new();
        let mut iter = s.split_whitespace();

        // position
        match iter.next() {
            Some(fen_pos) => b.read_fen_pos(fen_pos)?,
            None => return Err(Self::Err{description:format!("Invalid fen string '{}': no position supplied", s)})
        };

        // active color
        match iter.next() {
            Some("w") => b.current_player = Color::White,
            Some("b") => b.current_player = Color::Black,
            Some(col) => return Err(Self::Err{description:format!("Invalid fen string '{}': invalid active color '{}'", s, col)}),
            None => return Err(Self::Err{description:format!("Invalid fen string '{}': no active color specified", s)})
        };

        Ok(b)
    }
}

impl Board {
    fn read_fen_pos(&mut self, fen_pos: &str) -> Result<(), ParseFenError> {
        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        for (idx, c) in fen_pos.chars().enumerate() {
            if c == '/' {
                if file != 8 || rank == 0 { return Err(ParseFenError{description:format!("Invalid fen string '{}': didn't expect '/' at pos {}", fen_pos, idx)}); }

                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                let offset = c.to_digit(10).unwrap() as u8;
                if offset > 8 || file + offset > 8 { return Err(ParseFenError{description:format!("Invalid fen string '{}': invalid offset {} at pos {}", fen_pos, offset, idx)}); }

                file += offset;
            } else {
                if file >= 8 { return Err(ParseFenError{description:format!("Invalid fen string '{}': position goes out of bounds at pos {}", fen_pos, idx)}) }

                match Piece::from_fen_char(&c) {
                    Some(p) => self.put_piece(&Position::from(rank, file), Some(p)),
                    None => return Err(ParseFenError{description:format!("Invalid fen string '{}': '{}' at pos {} isn't a fen char", fen_pos, c, idx)})
                };

                file += 1;
            }
        }

        Ok(())
    }
}
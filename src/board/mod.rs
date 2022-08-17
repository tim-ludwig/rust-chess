pub mod position;
mod game_state;

use std::fmt::{Display, Formatter};
use std::ptr::write;
use crate::piece::{Color, Piece};
use position::Position;
use std::str::FromStr;
use crate::board::game_state::{CastlingState, GameState};

#[derive(Debug)]
pub struct Board {
    cells: [Option<Piece>; 64],
    state_stack: Vec<GameState>,
    current_player: Color,
    ply: u32,
}

impl Board {
    const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn new() -> Board {
        Self::STARTING_FEN.parse().expect("Invalid starting fen supplied")
    }

    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        self.cells[pos.idx()]
    }

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
        self.state_stack.last().expect("game state stack should not be empty")
    }

    fn get_state_mut(&mut self) -> &mut GameState  {
        self.state_stack.last_mut().expect("game state stack should not be empty")
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

macro_rules! parse_fen_error {
    ($($fmt:expr),*) => {
        Err(ParseFenError{description:format!($($fmt),*)})
    }
}

impl FromStr for Board {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Board, ParseFenError> {
        let mut b = Board {
            cells: [None; 64],
            state_stack: vec![GameState::new()],
            current_player: Color::White,
            ply: 0
        };
        let mut iter = s.split_whitespace();

        // position
        match iter.next() {
            Some(fen_pos) => b.read_fen_pos(fen_pos)?,
            None => return parse_fen_error!("Invalid fen string '{}': no position supplied", s)
        };

        // active color
        match iter.next() {
            Some("w") => b.current_player = Color::White,
            Some("b") => b.current_player = Color::Black,
            Some(col) => return parse_fen_error!("Invalid fen string '{}': invalid active color '{}'", s, col),
            None => return parse_fen_error!("Invalid fen string '{}': no active color specified", s)
        };

        // castling rights
        match iter.next() {
            Some(rights) => {
                match rights.parse() {
                    Ok(castling_state) => b.get_state_mut().castling = castling_state,
                    Err(ParseFenError{description}) => return parse_fen_error!("Invalid fen string '{}': {}", s, description)
                }
            },
            None => return parse_fen_error!("Invalid fen string '{}': no castling rights specified", s)
        };

        // en passant file
        match iter.next() {
            Some("-") => b.get_state_mut().en_passant_file = None,
            Some(square) => {
                match square.parse::<Position>() {
                    Ok(pos) => b.get_state_mut().en_passant_file = Some(pos.file),
                    Err(ParseFenError{description}) => return parse_fen_error!("Invalid fen string '{}': {}", s, description)
                }
            },
            None => return parse_fen_error!("Invalid fen string '{}': no en-passant file specified", s)
        };

        // fifty move clock
        match iter.next() {
            Some(count_str) => {
                match count_str.parse::<u8>() {
                    Ok(count) => b.get_state_mut().fifty_move_counter = count,
                    Err(_) => return parse_fen_error!("Invalid fen string '{}': invalid fifty move count '{}'", s, count_str),
                }
            }
            None => return parse_fen_error!("Invalid fen string '{}': no fifty move count specified", s)
        }

        // current ply.
        // FEN stores the move counter (not the ply count), starting at 1, so we need to adjust a little bit
        match iter.next() {
            Some(count_str) => {
                match count_str.parse::<u32>() {
                    Ok(count) => {
                        let mut ply = (count - 1) * 2;
                        if b.current_player == Color::Black { ply += 1; }

                        b.ply = ply;
                    },
                    Err(_) => return parse_fen_error!("Invalid fen string '{}': invalid move count '{}'", s, count_str),
                }
            }
            None => return parse_fen_error!("Invalid fen string '{}': no move count specified", s)
        }

        Ok(b)
    }
}

impl Board {
    fn read_fen_pos(&mut self, fen_pos: &str) -> Result<(), ParseFenError> {
        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        for (idx, c) in fen_pos.chars().enumerate() {
            if c == '/' {
                if file != 8 || rank == 0 {
                    return parse_fen_error!("Invalid fen string '{}': didn't expect '/' at pos {}", fen_pos, idx);
                }

                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                let offset = c.to_digit(10).unwrap();
                if file + offset > 8 {
                    return parse_fen_error!("Invalid fen string '{}': invalid offset {} at pos {}", fen_pos, offset, idx);
                }

                file += offset as u8;
            } else {
                if file >= 8 {
                    return parse_fen_error!("Invalid fen string '{}': position goes out of bounds at pos {}", fen_pos, idx)
                }

                match Piece::from_fen_char(&c) {
                    Some(p) => self.put_piece(&Position::from(rank, file), Some(p)),
                    None => return parse_fen_error!("Invalid fen string '{}': '{}' at pos {} isn't a fen char", fen_pos, c, idx)
                };

                file += 1;
            }
        }

        Ok(())
    }
}
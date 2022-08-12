use crate::piece::Piece;
use crate::position::Position;
use std::str::FromStr;

#[derive(Debug)]
pub struct Board {
    cells: [Cell; 64]
}

type Cell = Option<Piece>;

impl Board {
    pub fn new() -> Board {
        Board{ cells: [None; 64] }
    }

    pub fn at(&self, pos: Position) -> Cell { self.cells[pos.idx()] }
}

#[derive(Debug)]
pub struct ParseFenError {
    description: String
}

impl FromStr for Board {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Board, Self::Err> {
        let mut b = Board::new();
        let mut iter = s.split_whitespace();

        let fen_pos = match iter.next() {
            Some(fen_pos) => fen_pos,
            None => return Err(Self::Err{description:format!("Invalid fen string '{}': no position supplied", s)})
        };

        let mut rank = 7;
        let mut file = 0;

        for c in fen_pos.chars() {
            match c {
                '/' => {
                    if file != 7 { return Err(Self::Err{description:format!("Invalid fen string '{}': didn't expect '/'", s)}); }

                    rank -= 1;
                    file = 0;
                },
                '1'..='8' => match c.to_digit(10) {
                    Some(n) => file += n,
                    None => panic!("")
                },
                'K' | 'k' |
                'Q' | 'q' |
                'R' | 'r' |
                'B' | 'b' |
                'N' | 'n' |
                'P' | 'p' => todo!(),
                _ => return Err(Self::Err{description:format!("Invalid fen string '{}': invalid char in position", s)})
            }
        }

        Ok(b)
    }
}
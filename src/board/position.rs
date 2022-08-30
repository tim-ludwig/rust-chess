use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::board::ParseFenError;

#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub struct Position {
    pub rank: u8,
    pub file: u8
}

impl From<(u8, u8)> for Position {
    fn from((r, f): (u8, u8)) -> Self {
        Position::from(r, f)
    }
}

impl From<usize> for Position {
    fn from(idx: usize) -> Self {
        Position::from((idx / 8) as u8, (idx % 8) as u8)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (b'a' + self.file) as char)?;
        write!(f, "{}", (b'1' + self.rank) as char)?;

        Ok(())
    }
}

impl Position {
    pub fn from(r: u8, f: u8) -> Self {
        Position{ rank: r, file: f }
    }

    pub fn idx(&self) -> usize {
        (self.rank * 8 + self.file) as usize
    }
}

impl FromStr for Position {
    type Err =  ParseFenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match (chars.next(), chars.next(), chars.next()) {
            (Some(f), Some(r), None) => {
                if !('1' <= r && r <= '8' && 'a' <= f && f <= 'h') {
                    Err(ParseFenError{description:format!("invalid position '{}'", s)})
                } else {
                    Ok(Position::from(r as u8 - b'1', f as u8 - b'a'))
                }
            },
            _ => Err(ParseFenError{description:format!("invalid position '{}'", s)})
        }
    }
}
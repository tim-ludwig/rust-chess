use std::fmt::{Display, Formatter};

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
        ((idx / 8) as u8, (idx % 8) as u8).into()
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
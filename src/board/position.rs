use std::fmt::{Display, Formatter};

pub struct Position {
    pub rank: usize,
    pub file: usize
}

impl From<(usize, usize)> for Position {
    fn from((r, f): (usize, usize)) -> Self {
        Position{ rank: r, file: f }
    }
}

impl From<usize> for Position {
    fn from(idx: usize) -> Self {
        (idx / 8, idx % 8).into()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (b'a' + self.file as u8) as char)?;
        write!(f, "{}", (b'1' + self.rank as u8) as char)?;

        Ok(())
    }
}

impl Position {
    pub fn idx(&self) -> usize { self.rank * 8 + self.file }
}
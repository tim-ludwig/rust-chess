use std::str::FromStr;

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

impl Position {
    pub fn idx(&self) -> usize { self.rank * 8 + self.file }
}
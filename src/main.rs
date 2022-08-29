mod piece;
mod board;

use board::Board;
use crate::board::representation::BoardRepr;

fn main() {
    let b: Board = Board::new();

    println!("{}", b);
}

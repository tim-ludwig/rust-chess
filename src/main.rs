mod piece;
mod board;
mod position;

use board::Board;
use crate::position::Position;

fn main() {
    let b: Board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".parse().unwrap();

    dbg!(b);
}

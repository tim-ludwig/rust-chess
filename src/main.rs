mod board;
mod piece;

use board::Board;

fn main() {
    let b: Board = Board::new();

    println!("{}", b);
}

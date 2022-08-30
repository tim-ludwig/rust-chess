mod piece;
mod board;

use board::Board;

fn main() {
    let b: Board = Board::new();

    println!("{}", b);
}

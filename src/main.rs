mod piece;
mod board;

use board::Board;

fn main() {
    let b: Board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3".parse().unwrap();

    println!("{}", b);
}

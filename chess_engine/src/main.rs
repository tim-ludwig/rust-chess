mod board;
mod piece;

use chess_terminal_ui::terminal_ui::move_printhead_upwards;
use board::Board;

fn main() {
    let b: Board = "r1b1kb1r/pppp1ppp/5q2/4n3/3KP3/2N3PN/PPP4P/R1BQ1B1R b kq - 0 1".parse().unwrap_or_else(|_| Board::new());

    print!("{}", b);
    move_printhead_upwards();
    println!("{}", b);

}

use chess_engine::board::Board;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod terminal_ui {
    pub fn move_printhead_upwards() {
        print!("\x1b[17A");
    }
}

fn main() {
    let b: Board = "r1b1kb1r/pppp1ppp/5q2/4n3/3KP3/2N3PN/PPP4P/R1BQ1B1R b kq - 0 1".parse().unwrap_or_else(|_| Board::new());

    print!("{}", b);
    terminal_ui::move_printhead_upwards();
    println!("{}", b);

}
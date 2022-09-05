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

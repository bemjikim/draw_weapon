use std::io::{self, Write};
use crate::enums::input_message::input_message;

pub struct InputView;

impl InputView {
    pub fn input_number() -> usize {
        print!("{}", input_message::INPUT_PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap_or(0)
    }

    pub fn show_back_option() {
        println!("{}", input_message::BACK_OPTION);
    }
}

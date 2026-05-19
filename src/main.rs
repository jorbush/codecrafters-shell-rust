#[allow(unused_imports)]
use std::io::{self, Write};
mod command;

use command::Command;

fn main() {
    // REPL
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match Command::from_str(input) {
            Some(command) => {
                command.execute();
                continue;
            }
            None => println!("{}: command not found", input),
        }
    }
}

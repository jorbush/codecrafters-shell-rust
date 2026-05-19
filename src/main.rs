#[allow(unused_imports)]
use std::io::{self, Write};
mod command;
mod utils;

use command::Command;

use crate::utils::is_executable;

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

        let (command, params) = match input.split_once(' ') {
            Some((command, params)) => (command, params),
            None => (input, ""),
        };
        match Command::from_str(command) {
            Some(command) => {
                command.execute(params);
            }
            None => match is_executable(command) {
                true => {
                    println!(
                        "Program was passed with {} args (including program name).",
                        input.split_whitespace().count()
                    );
                    continue;
                }
                false => println!("{}: not found", command),
            },
        }
    }
}

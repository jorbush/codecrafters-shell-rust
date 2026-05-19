#[allow(unused_imports)]
use std::io::{self, Write};
mod command;
mod utils;

use command::Command;

use crate::utils::{find_exec_dir, is_executable};

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
                continue;
            }
            None => match find_exec_dir(command) {
                Some(dir) => {
                    std::process::Command::new(command)
                        .args(params.split_whitespace())
                        .spawn()
                        .expect("Failed to execute command");
                    println!();
                    continue;
                }
                None => println!("{}: not found", command),
            },
        }
    }
}

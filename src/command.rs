#[derive(Debug, PartialEq)]
pub enum Command {
    Echo,
    Exit,
}

impl Command {
    pub fn from_str(s: &str) -> Option<Command> {
        match s {
            "echo" => Some(Command::Echo),
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }

    pub fn execute(&self, params: &str) -> () {
        match self {
            Command::Echo => println!("{}", params),
            Command::Exit => std::process::exit(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Command::from_str("exit"), Some(Command::Exit));
        assert_eq!(Command::from_str("invalid"), None);
    }

    #[test]
    fn test_execute_echo() {
        let command = Command::Echo;
        let params = "Hello, world!";
        command.execute(params);
    }
}

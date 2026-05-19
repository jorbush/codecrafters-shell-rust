#[derive(Debug, PartialEq)]
pub enum Command {
    Exit,
}

impl Command {
    pub fn from_str(s: &str) -> Option<Command> {
        match s {
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }

    pub fn execute(&self) -> () {
        match self {
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
}

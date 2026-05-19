use crate::utils::is_executable;

#[derive(Debug, PartialEq)]
pub enum Command {
    Type,
    Echo,
    Exit,
}

impl Command {
    pub fn from_str(s: &str) -> Option<Command> {
        match s {
            "type" => Some(Command::Type),
            "echo" => Some(Command::Echo),
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Command::Type => "type",
            Command::Echo => "echo",
            Command::Exit => "exit",
        }
    }

    pub fn execute(&self, params: &str) {
        match self {
            Command::Type => match Command::from_str(params) {
                Some(command) => println!("{} is a shell builtin", command.to_string()),
                None => {
                    let path = std::env::var("PATH").unwrap_or_default();
                    let directories: Vec<&str> = path.split(':').collect();
                    for dir in directories {
                        if is_executable(&format!("{}/{}", dir, params)) {
                            println!("{} is {}/{}", params, dir, params);
                            return;
                        }
                    }
                    println!("{}: not found", params);
                }
            },
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
    fn test_to_string() {
        let command = Command::Echo;
        assert_eq!(command.to_string(), "echo");
    }

    // #[test]
    // fn test_execute_exit() {
    //     let command = Command::Exit;
    //     std::panic::catch_unwind(|| command.execute("")).unwrap_err();
    // }

    #[test]
    fn test_execute_echo() {
        let command = Command::Echo;
        let params = "Hello, world!";
        command.execute(params);
        // todo: assert that the output is correct
    }

    #[test]
    fn test_execute_type_builtin() {
        let command = Command::Type;
        for cmd in [Command::Type, Command::Echo, Command::Exit] {
            let params = cmd.to_string();
            command.execute(params);
            // todo: assert that the output is correct
        }
    }

    #[test]
    fn test_execute_type_not_found() {
        let command = Command::Type;
        let params = "invalid";
        command.execute(params);
        // todo: assert that the output is correct
    }

    #[test]
    fn test_execute_type_local_executable_files() {
        let command = Command::Type;
        let params = "ls";
        command.execute(params);
        // todo: assert that the output is correct
    }
}

use mysql::consts::GeometryType;
use serde::de::Expected;

#[derive(Debug)]
pub enum Error {
    Misc(String),
    IO(std::io::Error),
    InvallidCommand(String),
    NoCommand(String)
}

#[derive(Debug, PartialEq)]
enum Command {
    Gardern
}

pub fn parser_args() -> Result<(), Error> {
    let command = std::env::args().nth(1).expect("Expected command");
    match match_command(command) {
        Ok(_) => {},
        Err(err) => return Err(err) 
    }

    Ok(())
}

fn match_command(command: String) -> Result<Command, Error> {
    match command.as_str() {
        "garden" => Ok(Command::Gardern),
        "" => Err(Error::NoCommand("Expected command".to_string())),
        _ => {
            let msg = format!("{command}, is not a valid command");
            Err(Error::InvallidCommand(msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::match_command;
    use super::Error;

    #[test]
    fn test_invalid_cmd() {
        let command = "invalid".to_string();
        let res = match_command(command).unwrap_err();
        match res {
            Error::InvallidCommand(_) => {}
            _ => panic!("Expected Error::InvallidCommand. Instead got: {:?}", res)
        }
    }

    #[test]
    fn test_no_cmd() {
        let command = "".to_string();
        let res = match_command(command).unwrap_err();
        match res {
            Error::NoCommand(_) => {}
            _=> panic!("Expected ErrorNoCommand. Instead got: {:?}", res)
        }
    }

    #[test]
    fn test_valid_command() {
        let command = "garden".to_string();
        match_command(command).unwrap();
    }
}
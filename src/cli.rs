use mysql::Pool;

mod garden;
mod container;
mod container_img;
use crate::database::connection;

#[derive(Debug)]
pub enum Error {
    Misc(String),
    IO(std::io::Error),
    InvallidCommand(String),
    NoCommand(String)
}

#[derive(Debug, PartialEq)]
enum Command {
    Garden,
    Container
}

impl Command {
    fn run(&self, args: Vec<String>, pool: Pool) -> Result<(), Error> {
        let pool = pool.clone();

        match self {
            Command::Garden => garden::garden_cmd(&args, pool),
            Command::Container => container::container_cmd(&args, pool)
        }

        Ok(())
    }
}

pub fn parser_args() -> Result<(), Error> {
    let conn = connection::Connection::new(None);
    let pool = conn.get_pool();

    let command = std::env::args().nth(1).expect("Expected command");
    match match_command(command) {
        Ok(cmd) => {
            let (_, args): (Vec<usize>, Vec<String>) = std::env::args()
                .enumerate()
                .filter(|(i, _): &(usize, String)| *i>1)
                .collect();
            return cmd.run(args, pool)
        },
        Err(err) => return Err(err) 
    }
}

fn match_command(command: String) -> Result<Command, Error> {
    match command.as_str() {
        "garden" => Ok(Command::Garden),
        "container" => Ok(Command::Container),
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
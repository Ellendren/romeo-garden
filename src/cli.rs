use mysql::Pool;
use colored::Colorize;

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
    Container,
    Start,
    Exit
}

impl Command {
    fn run(&self, args: Vec<String>, pool: Pool) -> Result<(), Error> {
        let pool = pool.clone();

        match self {
            Command::Garden => garden::garden_cmd(&args, pool),
            Command::Container => container::container_cmd(&args, pool),
            Command::Exit => {
                println!("{}", "Goodbye".blue());
                std::process::exit(0);
            }
            Command::Start => {
                println!("{}", "Welcome to Romeo Garden!".blue());

                let mut cmd_str = String::new();
                while cmd_str.to_lowercase() != format!("exit") {
                    cmd_str = format!("");
                    println!("Enter command (or exit to end):");
                    match std::io::stdin().read_line(&mut cmd_str){
                        Ok(_) => {},
                        Err(e) => eprint!("{}: {:?}", "Input Error".red(), e)
                    };

                    //remove endline and split commands and oprtions
                    cmd_str.remove(cmd_str.len()-1);

                    let mut between_quotes = false;
                    let args: Vec<String> = cmd_str
                        .clone()
                        .split(|c| {
                            if c == '"' {
                                between_quotes = !between_quotes;
                                return true;
                            }
                            else if c == ' '  && !between_quotes{
                                return true;
                            }
                            else {
                                false
                            }
                        })
                        .map(|x| x.to_string())
                        .filter(|s| s.len() != 0)
                        .collect();

                    match match_command(args[0].clone()){
                        Ok(cmd) => {
                            cmd.run(args[1..].to_vec(), pool.clone()).unwrap();
                        },
                        Err(e) => return Err(e)
                    }
                }
            }
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
        "start" => Ok(Command::Start),
        "exit" => Ok(Command::Exit),
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
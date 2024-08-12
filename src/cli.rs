use mysql::consts::GeometryType;

#[derive(Debug)]
pub enum Error {
    Misc(String),
    IO(std::io::Error),
    InvallidCommand(String)
}

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
        _ => {
            let msg = format!("{command}, is not a valid command");
            Err(Error::InvallidCommand(msg))
        }
    }
}
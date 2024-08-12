use std::fmt::format;

use colored::Colorize;

mod database;
use cli::parser_args;
use database::connection::Connection;
mod cli;

fn main() {
    match parser_args() {
        Ok(_) => {},
        Err(err) => {
            let msg = format!("{}: {:?}", "Err".red().bold(), err);
            eprintln!("{}", msg);
        }
    };
}

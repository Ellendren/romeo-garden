use serde::Deserialize;
use std::{fs::File, io::Read};
use mysql::*;

#[derive(Deserialize, Debug)]
struct Database {
    username: String,
    password: String,
    database_name: String,
    port: String,
    host: String
}

#[derive(Deserialize, Debug)]
struct  Config {
    database: Database
}

#[derive(Debug)]
pub struct Connection {
    config: Config,
    pool: Option<Pool>
}

impl Connection {
    pub fn new(custom_path: Option<&str>) -> Self{
        let path = match custom_path {
            Some(val) => val,
            None => "./config.toml"
        };

        let mut file = File::open(path).unwrap();

        let mut config_str = String::new();
        file.read_to_string(&mut config_str);

        let config: Config = toml::from_str(&config_str).unwrap();

        Connection {
            config: config,
            pool: None
        }
    }
}
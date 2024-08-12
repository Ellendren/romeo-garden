use serde::Deserialize;
use std::{fs::File, io::Read};
use mysql::*;
use colored::Colorize;

#[derive(Deserialize, Debug)]
struct Database {
    username: String,
    password: String,
    database_name: String,
    port: String,
    host: String
}

impl Database {
    fn get_url_mysql(&self) -> String{
        let url = "mysql://".to_string()
            + &self.username + ":"
            + &self.password + "@"
            + &self.host + ":"
            + &self.port + "/"
            + &self.database_name;
        
        url
    }
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

    //init new Pool if one doesn't exist for this version of Connection
    //panics id conection can't make a new pool
    pub fn get_pool(mut self) -> Pool {
        match self.pool {
            Some(pool) => return pool,
            None => {}
        }

        let url = self.config.database.get_url_mysql();

        self.pool = match Pool::new(url.as_str()) {
            Ok(pool) => Some(pool),
            Err(e) => {
                let err_str = format!("{} {}: {:?}",
                     "Error creating connection pool for"
                        .bold()
                        .red(), 
                     self.config.database.database_name
                        .bold()
                        .red(),
                     e);
                panic!("{}", err_str);
            }
        };

        self.pool.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Connection;

    // fails of database connection is not succesful
    #[test]
    fn connection_succes() {
        let conn = Connection::new(None);
        conn.get_pool();
    }
}
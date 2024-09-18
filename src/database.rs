pub mod connection;
pub mod garden_controller;
pub mod container_controller;
pub mod plant_controller;

use mysql::{
    prelude::Queryable, Error, Pool
};

pub fn query_drop(pool: Pool, query: &str) -> Result<(), Error> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return Err(e)
    };
    conn.query_drop(query)
}
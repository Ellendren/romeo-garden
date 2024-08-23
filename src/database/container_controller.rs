use mysql::{
    prelude::Queryable, Error, Pool
};
use serde::{Deserialize, Serialize};

use super::query_drop;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContainerController{
    container_name: String,
    garden_name: String,
    ctype: String,
    width: f32,
    length: f32,
    height: f32,
    volume: f32,
}

impl ContainerController {

    // creates new container. default ctype: bed
    pub fn new (
        container_name: String,
        garden_name: String,
        ctype: Option<String>,
        width: Option<f32>,
        length:  Option<f32>,
        height:  Option<f32>,
        volume:  Option<f32>,
    ) -> Self {
        let ctype = match ctype {
            Some(val) => val,
            None => "bed".to_string()
        };
        let width = match width {
            Some(val) => val,
            None => 0.0
        };
        let length = match length {
            Some(val) => val,
            None => 0.0
        };
        let height = match height {
            Some(val) => val,
            None => 0.0
        };
        let volume = match volume {
            Some(val) => val,
            None => width * length * height
        };

        ContainerController {
            container_name,
            garden_name,
            ctype,
            width,
            length,
            height,
            volume
        }
    }

    pub fn container_name(&self) -> String {
        self.container_name.clone()
    }

    pub fn garden_name(&self) -> String {
        self.garden_name.clone()
    }

    pub fn length(&self) -> f32 {
        self.length
    }
    
    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn view_container(self, pool: Pool) -> Result<Vec<ContainerController>, Error> {
        view_container(pool, &self.container_name, &self.garden_name)
    }
}

pub fn view_container(pool: Pool, container_name: &str, garden_name: &str) -> Result<Vec<ContainerController>, Error> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return Err(e)
    };

    let query = format!("CALL view_container('{container_name}', '{garden_name}')");
    conn.query_map(query, 
        |(container_name, garden_name, ctype, width, length, height, volume)| {
            ContainerController {
                container_name,
                garden_name,
                ctype,
                width,
                length,
                height,
                volume
            }
    })
}

// pub fn add_container_bed(pool: Pool, cname: &str, gname: &str) -> Result<(), Error> {
//     let query = format!("CALL add_bed('')");
//     query_drop(pool, &query)
// }

#[cfg(test)]
mod tests {
    use super::view_container;
    use super::super::connection;

    #[test]
    fn view_container_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        let res = view_container(pool.clone(), "", "");
        println!("{:?}", res);
        assert!(res.is_ok())
    }
}
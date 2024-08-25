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
    width: f64,
    length: f64,
    height: f64,
    volume: f64,
}

impl ContainerController {

    // creates new container. default ctype: bed
    pub fn new (
        container_name: String,
        garden_name: String,
        ctype: Option<String>,
        width: Option<f64>,
        length:  Option<f64>,
        height:  Option<f64>,
        volume:  Option<f64>,
    ) -> Self {
        let ctype = match ctype {
            Some(val) => val,
            None => "bed".to_string()
        };
        let width = match width {
            Some(val) => val,
            None => 12.0
        };
        let length = match length {
            Some(val) => val,
            None => 12.0
        };
        let height = match height {
            Some(val) => val,
            None => 12.0
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

    pub fn length(&self) -> f64 {
        self.length
    }
    
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn view_container(self, pool: Pool) -> Result<Vec<ContainerController>, Error> {
        view_container(pool, &self.container_name, &self.garden_name)
    }

    pub fn add_container_bed(self, pool: Pool) -> Result<(), Error> {
        add_container_bed(
            pool, 
            &self.container_name,
            &self.garden_name,
            self.width, 
            self.length, 
            self.height, 
            self.volume
        )
    }

    pub fn drop_container(self, pool: Pool) -> Result<(), Error> {
        drop_container(pool, &self.container_name, &self.garden_name)
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

pub fn add_container_bed(
    pool: Pool, 
    cname: &str, 
    gname: &str,
    width: f64,
    length: f64,
    height: f64,
    volume: f64
) -> Result<(), Error> {
    let query = format!("CALL add_bed(
            \"{cname}\",
            \"{gname}\",
            \"{width}\",
            \"{length}\",
            \"{height}\",
            \"{volume}\")"
        );
    query_drop(pool, &query)
}

pub fn drop_container(pool: Pool, cname: &str, gname: &str) -> Result<(), Error> {
    let query = format!("CALL drop_container(\"{cname}\", \"{gname}\")");
    query_drop(pool, &query)
}

#[cfg(test)]
mod tests {
    use super::{
        view_container,
        ContainerController
    };
    use super::super::connection;

    const TEST_CNAME: &str = "container_name";
    const TEST_GNAME: &str = "garden_name";

    #[test]
    fn view_container_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        let res = view_container(pool.clone(), "", "");
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn add_container_bed_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        //create garden for tests
        crate::database::garden_controller::add_garden(pool.clone(), TEST_GNAME).unwrap();

        let test_container: ContainerController = ContainerController::new(
            TEST_CNAME.to_string(),
            TEST_GNAME.to_string(),
            None, 
            None, 
            None, 
            None, 
            None
        );

        let res = test_container.add_container_bed(pool.clone());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn drop_container() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        let test_container: ContainerController = ContainerController::new(
            TEST_CNAME.to_string(),
            TEST_GNAME.to_string(),
            None, 
            None, 
            None, 
            None, 
            None
        );

        let res = test_container.drop_container(pool.clone());
        println!("{:?}", res);

        //remove test garden
        crate::database::garden_controller::remove_garden(pool.clone(), TEST_GNAME).unwrap();

        assert!(res.is_ok());
    }
}
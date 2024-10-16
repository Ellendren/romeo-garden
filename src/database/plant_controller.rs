use std::option;

use mysql::{
    prelude::Queryable, Error, Pool
};
use serde::{Deserialize, Serialize};
use chrono::{offset::{self, Local}, NaiveDate};

use super::query_drop;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Plant {
    plant_id: u64,
    date_planted: NaiveDate,
    container_name: String,
    garden_name: String,
    location: f64,
    t: Type
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Type {
    species: String,
    variety: String,
    size: f64,
    info_file: String
}

impl Plant {
    pub fn new(
        plant_id: Option<u64>,
        date_planted: Option<NaiveDate>,
        container_name: Option<String>,
        garden_name: Option<String>,
        location: Option<f64>,
        species: Option<String>,
        variety: Option<String>,
        size: Option<f64>,
        info_file: Option<String>
    ) -> Self  {
        Plant {
            plant_id: plant_id.unwrap_or(0),
            date_planted: date_planted.unwrap_or(NaiveDate::from(Local::now().date_naive())),
            container_name: container_name.unwrap_or("none".to_string()),
            garden_name: garden_name.unwrap_or("none".to_string()),
            location: location.unwrap_or(0.0),
            t: Type::new(species, variety, size, info_file)
        }
    }

    pub fn add(&self, pool: Pool) -> Result<(), Error> {
        add_plant(
            pool, 
            self.container_name.clone(),
            self.garden_name.clone(),
            self.location,
            self.t.species.clone(),
            self.t.variety.clone()
        )
    }

    pub fn drop(&self, pool: Pool) -> Result<(), Error> {
        drop_plant(pool, self.plant_id)
    }

    pub fn view(&self, pool: Pool) -> Result<Vec<Plant>, Error> {
        view_plant(pool, self.plant_id)
    }
}

impl Type {
    pub fn new(
        species: Option<String>,
        variety: Option<String>,
        size: Option<f64>,
        info_file: Option<String>
    ) -> Self {
        Type {
            species: species.unwrap_or("Other".to_string()),
            variety: variety.unwrap_or("Other".to_string()),
            size: size.unwrap_or(12.0),
            info_file: info_file.unwrap_or("none".to_string()),
        }
    }
}

pub fn add_plant(
    pool: Pool, 
    container_name: String,
    garden_name: String,
    location: f64,
    species: String,
    variety: String
) -> Result<(), Error> {
    let query = format!("CALL add_plant(
        \"{container_name}\",
        \"{garden_name}\",
        \"{location}\",
        \"{species}\",
        \"{variety}\")"
    );
    query_drop(pool, &query)
}

pub fn drop_plant(pool: Pool, plant_id: u64) -> Result<(), Error> {
    let query = format!("CALL drop_plant(\"{plant_id}\")");
    query_drop(pool, &query)
}

pub fn view_plant(pool: Pool, plant_id: u64) -> Result<Vec<Plant>, Error> {
    let query = format!("CALL view_plant(\"{plant_id}\")");
    
    plant_query_map(pool, query)
}

pub fn get_plants_container(pool: Pool, container_name: String, garden_name: String) -> Result<Vec<Plant>, Error> {
    let query = format!("CALL get_plants_container(\"{container_name}\", \"{garden_name}\")");
    
    plant_query_map(pool, query)
}

fn plant_query_map(pool: Pool, query: String) -> Result<Vec<Plant>, Error> {
    let mut conn = match pool.get_conn()   {
        Ok(conn) => conn,
        Err(e) => return Err(e)
    };

    conn.query_map(
        query,
        |(
            plant_id,
            date_planted,
            container_name,
            garden_name,
            location,
            species,
            variety,
            size,
            info_file
        )| {    
            let date_planted: String = date_planted;
            let date_planted = NaiveDate::parse_from_str(date_planted.as_str(), "%Y-%m-%d")
                .unwrap_or(offset::Local::now().date_naive());
            Plant {
                plant_id,
                date_planted,
                container_name,
                garden_name,
                location,
                t: Type {
                    species,
                    variety,
                    size,
                    info_file
                }
            }
        }
    )
}

pub fn add_plant_type(
    pool: Pool, 
    species: String, 
    variety: String,
    size: f64,
    file_path: String
) -> Result<(), Error> {
    let query = format!("CALL add_plant_type(
        \"{species}\",
        \"{variety}\",
        \"{size}\",
        \"{file_path}\"
    )");
    query_drop(pool, &query)
}

pub fn drop_plant_type(pool: Pool, species: String, variety: String) -> Result<(), Error> {
    let query = format!("CALL drop_plant_type(\"{species}\", \"{variety}\")");
    query_drop(pool, &query)
}

#[cfg(test)]
mod tests {
    use super::{
        add_plant_type, drop_plant_type, Plant, view_plant
    };
    use mysql::Pool;
    use crate::database::{connection, plant_controller::get_plants_container};

    const TEST_CNAME: &str = "container_name";
    const TEST_GNAME: &str = "garden_name";
    const TEST_LOCATION: f64 = 0.0;
    const TEST_SNAME: &str = "species_name";
    const TEST_VNAME: &str = "variety_name";

    fn plant_type_setup(pool: Pool) {
        crate::database::garden_controller::add_garden(pool.clone(), TEST_GNAME).unwrap();
        crate::database::container_controller::add_container_bed(
            pool.clone(), 
            TEST_CNAME, 
            TEST_GNAME, 
            0.0, 
            0.0, 
            0.0, 
            0.0
        ).unwrap();
        super::add_plant_type(
            pool.clone(), 
            TEST_SNAME.to_string(), 
            TEST_VNAME.to_string(), 
            0.0, 
            String::new()
        ).unwrap();
    }

    fn plant_type_cleanup(pool: Pool) {
        crate::database::container_controller::drop_container(pool.clone(), TEST_CNAME, TEST_GNAME).unwrap();
        crate::database::garden_controller::remove_garden(pool.clone(), TEST_GNAME).unwrap();
        super::drop_plant_type(pool, TEST_SNAME.to_string(), TEST_VNAME.to_string()).unwrap();
    }

    #[test]
    fn test_add_drop_plant_type() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        //add test
        let res = add_plant_type(
            pool.clone(), 
            TEST_SNAME.to_string(), 
            TEST_VNAME.to_string(), 
            0.0, 
            String::new()
        );
        println!("{:?}", res);
        assert!(res.is_ok());

        //drop test
        let res = drop_plant_type(pool.clone(),  TEST_SNAME.to_string(), TEST_VNAME.to_string());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_add_drop_plant() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        // setup
        plant_type_setup(pool.clone());

        let plant = Plant::new(
            None, 
            None, 
            Some(TEST_CNAME.to_string()), 
            Some(TEST_GNAME.to_string()), 
            Some(TEST_LOCATION), 
            Some(TEST_SNAME.to_string()), 
            Some(TEST_VNAME.to_string()), 
            None, 
            None
        );

        // test add
        let res = plant.add(pool.clone());
        println!("{:?}", res);
        assert!(res.is_ok());

        // test drop
        let res = plant.drop(pool.clone());
        println!("{:?}", res);
        assert!(res.is_ok());

        // cleanup
        plant_type_cleanup(pool);
    }

    #[test]
    fn view_plant_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        let res = view_plant(pool, 0);
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn get_plants_container_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();

        let res = get_plants_container(pool, String::new(), String::new());
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}
use mysql::{
    prelude::Queryable, Error, Pool
};

pub fn view_gardens(pool: Pool) -> Result<Vec<String>, Error>{
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return Err(e)
    };
        
    match conn.query("SELECT * FROM Garden") {
        Ok(res) => {
            Ok(res)
        },
        Err(e) => Err(e)
    }
}

pub fn add_garden(pool: Pool, gname: &str) -> Result<(), Error> {
    let query = format!("CALL add_garden('{gname}')");
    query_drop(pool, &query)
}

pub fn remove_garden(pool: Pool, gname: &str) -> Result<(), Error> {
    let query = format!("CALL drop_garden('{gname}')");
    query_drop(pool, &query)
}

pub fn new_name_garden(pool: Pool, old_name: &str, new_name: &str) -> Result<(), Error>{
    let query = format!("CALL new_name_garden('{old_name}', '{new_name}')");
    query_drop(pool, &query)
}

fn query_drop(pool: Pool, query: &str) -> Result<(), Error> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return Err(e)
    };
    conn.query_drop(query)
}

#[cfg(test)]
mod tests {
    use super::{
        view_gardens,
        add_garden,
        remove_garden,
        new_name_garden
    };
    use crate::database::connection;

    #[test]
    fn view_gardens_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool().clone();
        
        let res = view_gardens(pool);
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn add_garden_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();
        let gname = "Test";
        
        let res = add_garden(pool.clone(), &gname);
        println!("{:?}", res);
        assert!(res.is_ok());

        //clean up after test
        let pool = pool.clone();
        remove_garden(pool, &gname).unwrap();
    }

    #[test]
    fn remove_garden_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();
        let gname = "Test";

        //add garden for test
        remove_garden(pool.clone(), &gname).unwrap();
        
        let res = remove_garden(pool.clone(), &gname);
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn new_name_garden_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool();
        let gname = "Test";
        let gname_new = "TestNew";

        //add garden for test
        remove_garden(pool.clone(), &gname).unwrap();

        // test new name
        let res = new_name_garden(pool.clone(), &gname, &gname_new);
        println!("{:?}", res);
        assert!(res.is_ok());

        //cleanup
        remove_garden(pool.clone(), &gname_new).unwrap();
    }
}
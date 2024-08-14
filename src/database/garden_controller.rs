use mysql::{
    prelude::Queryable, Error, Pool
};

pub fn view_gardens(pool: Pool) -> Result<(), Error>{
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return Err(e)
    };
        
    match conn.query("SELECT * FROM Garden") {
        Ok(res) => {
            let res_str: Vec<String> = res;
            println!("{:?}", res_str);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::view_gardens;
    use crate::database::connection;

    #[test]
    fn view_gardens_test() {
        let conn = connection::Connection::new(None);
        let pool = conn.get_pool().clone();
        
        let res = view_gardens(pool);

        assert!(res.is_ok());
    }
}
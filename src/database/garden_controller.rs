use mysql::{
    prelude::Queryable, Error, Pool
};

pub fn view_gaden(pool: Pool) -> Result<(), Error>{
    let q_thread = std::thread::spawn(move || {
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
    });

    q_thread.join().unwrap()
}
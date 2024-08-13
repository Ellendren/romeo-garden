use mysql::Pool;

use crate::database::connection;
use crate::database::garden_controller::view_gaden;

pub fn garden_cmd(args: &Vec<String>) {
    let conn = connection::Connection::new(None);
    let pool = conn.get_pool();

    if args.len() == 0 {
        let pool = pool.clone();
        match view_gaden(pool) {
            Ok(_) => {},
            Err(e) => eprintln!("{:?}", e)
        };
    }
}
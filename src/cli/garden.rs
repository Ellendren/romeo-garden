use mysql::Pool;

use crate::database::garden_controller::view_gaden;

pub fn garden_cmd(args: &Vec<String>, pool: Pool) {
    if args.len() == 0 {
        match view_gaden(pool) {
            Ok(_) => {},
            Err(e) => eprintln!("{:?}", e)
        };
    }
}
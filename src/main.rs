use connection::Connection;

mod connection;

fn main() {
    let conn = Connection::new(None);
    conn.get_pool();
}

use postgres::{Connection as PostgresConnection, TlsMode};
use std::env;

use chrono::NaiveDate;
use crate::user::{
    UserReply,
};

pub fn establish_postgres_connection() -> PostgresConnection {
    // $POSTGRES=<aws> cargo run --release
    // $POSTGRES=<aws> target/release/tonic_server
    let database_url = env::var("POSTGRES").expect("It must exist and valid.");
    PostgresConnection::connect(database_url, TlsMode::None).unwrap()
}

// Separate it to Model/ or make another file for this if necessary.
pub fn query_list_of_users(postgres_conn: PostgresConnection) -> Vec<UserReply> {
    let mut v: Vec<UserReply> = Vec::new();
    // https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.query
    for row in &postgres_conn.query("SELECT * FROM users", &[]).unwrap() {
        let date_of_birth: NaiveDate = row.get(3);
        let user = UserReply {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            date_of_birth: date_of_birth.to_string(),
        };
        v.push(user);
    }
    v
}

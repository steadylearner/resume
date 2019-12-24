use postgres::{Connection as PostgresConnection, TlsMode};
use dotenv::dotenv;
use std::env;

pub fn establish_postgres_connection() -> PostgresConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PostgresConnection::connect(database_url, TlsMode::None).unwrap()
}
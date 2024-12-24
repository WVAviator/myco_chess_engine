use rusqlite::Connection;

pub mod build;
pub mod schema;

pub fn get_connection() -> Connection {
    Connection::open("./moves-db.db3").expect("failed to create database connection")
}

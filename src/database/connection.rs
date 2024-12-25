use std::{env, path::PathBuf};

use rusqlite::Connection;

pub fn get_connection() -> Connection {
    Connection::open(get_database_path()).expect("failed to create database connection")
}

fn get_database_path() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from("resources/myco.db3")
    } else {
        let mut exe_path = env::current_exe().expect("failed to get current executable path");
        exe_path.pop();
        exe_path.push("resources/myco.db3");
        exe_path
    }
}

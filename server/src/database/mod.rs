pub mod structs;

use crate::database::structs::StatusOfFuncation::{Fail, Success};
use clap::Error;
use lazy_static::lazy_static;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use uuid::uuid;

lazy_static! {
    static ref POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = {
        let manager = r2d2_sqlite::SqliteConnectionManager::file("data/db");
        r2d2::Pool::builder()
            .min_idle(Some(0))
            .max_size(100)
            .build(manager)
            .expect("POOL err")
    };
}
fn get() -> PooledConnection<SqliteConnectionManager> {
    let conn = POOL.get().expect("Err");
    return conn;
}
pub fn create_user(user: &mut structs::User) -> structs::StatusOfFuncation {
    user.id = Some(uuid::Uuid::new_v4().to_string());
    user.timestamp = Some(chrono::Utc::now().naive_utc().to_string());
    let conn = get();
    let result = conn.execute(
        "INSERT INTO Users (id, username,password,created_at) VALUES (? , ?\
    ,? , ? )",
        params![
            user.id.clone().unwrap().to_string(),
            user.username,
            user.password,
            user.timestamp.clone().unwrap().to_string()
        ],
    );
    match result {
        Ok(_) => Success,
        Err(e) => {
            println!("Error in create user query : {}", e);
            Fail
        }
    }
}
pub fn delete_user(user: &mut structs::User) -> structs::StatusOfFuncation {
    let conn = get();
    let result = conn.execute("DELETE FROM Users WHERE username = ?", params![
        user.username
    ]);
    match result {
        Ok(_) => Success,
        Err(e) => {
            println!("Error in delete user query : {}", e);
            Fail
        }
    }
}
pub fn create_file(file: &mut structs::File) -> structs::StatusOfFuncation {
    file.timestamp = chrono::Utc::now().naive_utc().to_string();
    let conn = get();
    let result = conn.execute(
        "INSERT INTO Files (Filename, owner,type,description , created_at) VALUES (? , ?\
    ,? , ? , ? )",
        params![
            file.Filename,
            file.owner,
            file.typeoffile,
            file.description,
            file.timestamp.to_string()
        ],
    );
    match result {
        Ok(_) => Success,
        Err(e) => {
            println!("Error in create file query : {}", e);
            Fail
        }
    }
}
pub fn delete_file(file: &mut structs::File) -> structs::StatusOfFuncation {
    let conn = get();
    let result = conn.execute("DELETE FROM Files WHERE Filename = ?", params![
        file.Filename
    ]);
    match result {
        Ok(_) => Success,
        Err(e) => {
            println!("Error in delete file query : {}", e);
            Fail
        }
    }
}

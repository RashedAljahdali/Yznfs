//let me explain this so it will ch if the file "init.init" exists if so it will
//check if the file contain 0 "the program env is not initialized" and it will
//initialize the env and mark "init.init" as 1 ("the program env is initialized")
//if there is no file "init.init" it will make it and recall itself
//note I'm using .unwarp() that means this function shall not fail or the program will panic
use crate::database::structs::StatusOfFuncation;
use crate::database::structs::StatusOfFuncation::Success;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn init() -> StatusOfFuncation {
    if fs::try_exists("init.init").await.unwrap() {
        if fs::read_to_string("init.init").await.unwrap() == "0" {
            fs::create_dir_all("files").await.unwrap();
            fs::create_dir_all("data").await.unwrap();
            fs::write("init.init", "1").await.unwrap();
            initdb().await;
            StatusOfFuncation::Success
        } else {
            Success
        }
    } else {
        fs::write("init.init", "0").await.unwrap();
        return Box::pin(init()).await;
    }
}
//This function initialize all the tables in our database
//note I'm using .unwarp() that means this function shall not fail or the program will panic
async fn initdb() {
    let mut temp = rusqlite::Connection::open_with_flags(
        "data/db",
        rusqlite::OpenFlags::SQLITE_OPEN_CREATE | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
    )
    .unwrap();
    temp.execute("PRAGMA foreign_keys = ON;", ()).unwrap();
    temp.execute(
        r#"CREATE TABLE Users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP)
    "#,
        (),
    )
    .unwrap();
    temp.execute(
        r#"CREATE TABLE Files (
    Filename TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    type TEXT,
    description TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES User(id) ON DELETE CASCADE)
    "#,
        (),
    )
    .unwrap();
    temp.execute(
        r#"CREATE TABLE sessions (
    owner TEXT NOT NULL,
    session_id TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES User(id) ON DELETE CASCADE)
    "#,
        (),
    )
    .unwrap();
}

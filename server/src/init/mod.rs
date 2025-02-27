//let me explain this so it will ch if the file "init.init" exists if so it will
//check if the file contain 0 "the program env is not initialized" and it will
//initialize the env and mark "init.init" as 1 ("the program env is initialized")
//if there is no file "init.init" it will make it and recall itself
//note I'm using .unwarp() that means this function shall not fail or the program will panic
pub fn init(){
if std::fs::exists("init.init").unwrap(){
     if (std::fs::read_to_string("init.init").unwrap() == "0") {
        std::fs::create_dir_all("files").unwrap();
        std::fs::create_dir_all("data").unwrap();
        std::fs::write("init.init", "1").unwrap();
         initdb();
    }
    else {
        return;
    }
}
    else {
        std::fs::write("init.init" , "0").unwrap();
        init()
    }
}
//This function initialize all the tables in our database
//note I'm using .unwarp() that means this function shall not fail or the program will panic
fn initdb(){
    let mut temp = rusqlite::Connection::open_with_flags("data/db" ,
                                                         rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                                                             | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
    temp.execute("PRAGMA foreign_keys = ON;", ()).unwrap();
    temp.execute(r#"CREATE TABLE user (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP)
    "# , ()).unwrap();
    temp.execute(r#"CREATE TABLE files (
    Filename TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    type TEXT,
    description TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES user(id) ON DELETE CASCADE)
    "# , ()).unwrap();
    temp.execute(r#"CREATE TABLE sessions (
    owner TEXT NOT NULL,
    session_id TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES user(id) ON DELETE CASCADE)
    "# , ()).unwrap();
}
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
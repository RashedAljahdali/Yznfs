
use crate::database::structs::{StatusOfFuncation, User};
use rocket::{launch, post, routes};

#[post("/api/create_user", format = "json", data = "<user_data>")]
fn createuser(mut user_data: rocket::serde::json::Json<User>) -> String {
    let status = user_data.new();
    match status {
        StatusOfFuncation::Success => format!(
            "User {} with Password {} created!",
            user_data.username, user_data.password
        ),
        StatusOfFuncation::Fail => format!(
            "User {} with Password {} Failed!",
            user_data.username, user_data.password
        ),
    }
}
#[delete("/api/delete_user", format = "json", data = "<user_data>")]
fn deleteuser(user_data: rocket::serde::json::Json<User>) -> String {
    let status = user_data.delete();
    match status {
        StatusOfFuncation::Success => format!(
            "User {} with Password {} delete it!",
            user_data.username, user_data.password
        ),
        StatusOfFuncation::Fail => format!(
            "User {} with Password {} delete failed!",
            user_data.username, user_data.password
        ),
    }
}
#[launch]
pub async fn rocket() -> _ {
    rocket::build().mount("/", routes![createuser , deleteuser])
}

use crate::database::create_user;
use crate::database::structs::{StatusOfFuncation, User};
use rocket::response::content::RawJson;
use rocket::response::status;
use rocket::{launch, post, routes};

#[post("/api/create_user", format = "json", data = "<user_data>")]
fn createuser(mut user_data: rocket::serde::json::Json<User>) -> String {
    let status = create_user(&mut user_data);
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
#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![createuser])
}

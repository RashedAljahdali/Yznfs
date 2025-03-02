use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct File {
    pub Filename: String,
    pub owner: String,
    pub typeoffile: String,
    pub description: String,
    pub timestamp: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub password: String,
    pub timestamp: Option<String>,
}
pub enum StatusOfFuncation {
    Success,
    Fail,
}

#[macro_use]
extern crate rocket;
mod database;
mod init;
mod network;

use database::structs::StatusOfFuncation;
use rusqlite::DropBehavior::Panic;
#[main]
async fn main() {
    let m = init::init().await;
    network::rocket().launch().await;
}

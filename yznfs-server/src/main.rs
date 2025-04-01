#[macro_use]
extern crate rocket;



mod database;
mod init;
mod network;


#[main]
async fn main() {
    init::init().await;
    network::rocket().await.launch().await.expect("Error in the launch");
}

use unftp_sbe_fs::ServerExt;
use config_file::FromConfigFile;
use rocket::serde::{Deserialize};
use libunftp::{auth, auth::AnonymousAuthenticator, Server};
use std::sync::Arc;
use unftp_auth_jsonfile::JsonFileAuthenticator;
#[derive(Deserialize)]
struct Config {
    path: String,
}

#[tokio::main]
pub async fn main() {
    println!("Starting FTP server for Clurd");
    let config = Config::from_config_file("./config.toml").unwrap();
    let authenticator = JsonFileAuthenticator::from_file(String::from("credentials.json")).expect("Error during reading credentials");
    // let server = libunftp::Server::with_fs(config.path)
    //     .greeting("Welcome to FTP Clurd")
    //     .passive_ports(50000..65535)
    //     .metrics()
    //     .authenticator(Arc::new(authenticator));
    let server = libunftp::Server::with_fs(config.path)
        .greeting("Welcome to FTP Clurd")
        .passive_ports(50000..65535)
        .metrics();
    server.listen("0.0.0.0:21").await;
}

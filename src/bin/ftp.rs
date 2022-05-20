use unftp_sbe_fs::ServerExt;
use config_file::FromConfigFile;
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize)]
struct Config {
    path: String,
}

#[tokio::main]
pub async fn main() {
    println!("Starting FTP server for Clurd");
    let config = Config::from_config_file("./config.toml").unwrap();
    let server = libunftp::Server::with_fs(config.path)
        .greeting("Welcome to FTP Clurd")
        .passive_ports(50000..65535);

    server.listen("0.0.0.0:2121").await;
}

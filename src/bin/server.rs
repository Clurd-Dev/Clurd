/*
This is the server-side of Clurd built with Rocket.rs
Andrea Canale 2022
Not very beautiful yet.
*/
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{FileServer, Options};
use rocket::http::Header;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request, Response};
use std::fs;
use config_file::FromConfigFile;
mod info;
mod ls;
mod io;
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("access-control-allow-origin", "*"));
        response.set_header(Header::new(
            "access-control-allow-methods",
            "GET, PATCH, OPTIONS, POST",
        ));
    }
}
#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Task<'r> {
    folder: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct RenameFile<'r> {
    folder: &'r str,
    new: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

struct SpaceFolder {
    total: String,
    available: String,
}

#[post("/", data = "<file>")]
fn remove(file: Json<Task<'_>>) -> &str {
    io::remove(file.folder)
}

#[post("/", data = "<rename_file>")]
fn rename(rename_file: Json<RenameFile<'_>>) -> String {
    println!("{}", rename_file.folder);
    let renamed = fs::rename(rename_file.folder, rename_file.new);
    io::rename(rename_file.folder, rename_file.new)
}

#[post("/", data = "<rename_file>")]
fn copy(rename_file: Json<RenameFile<'_>>) -> String {
    io::copy(rename_file.folder, rename_file.new)
}
#[post("/", data = "<rename_file>")]
fn movefs(rename_file: Json<RenameFile<'_>>) -> String {
   io::movefs(rename_file.folder, rename_file.new)
}



#[post("/", data = "<task>")]
fn space(task: Json<Task<'_>>) -> Json<io::SpaceFolder> {
    io::space(task.folder)
}
#[post("/", data = "<task>")]
fn files(task: Json<Task<'_>>) -> String {
   ls::list_file(task.folder)
}

#[derive(Deserialize)]
struct Config {
    path: String,
}

#[derive(Serialize)]
struct ConfigResponse { 
    path: String
 }

#[get("/")]
fn get_config() -> Json<ConfigResponse> {
    let config = Config::from_config_file("./config.toml").unwrap();
    Json(ConfigResponse {
        path: config.path
    })
}

#[get("/")]
fn get_info() -> Json<info::Information>{
    info::info()
}

#[launch]
fn rocket() -> _ {
    let config = Config::from_config_file("./config.toml").unwrap();
    let options = Options::Index | Options::DotFiles;
    rocket::build()
        .mount("/getfiles", routes![files])
        .attach(Cors)
        .mount("/", FileServer::from("./"))
        .mount("/", FileServer::new(config.path, options).rank(-1))
        .mount("/remove", routes![remove])
        .mount("/space", routes![space])
        .mount("/rename", routes![rename])
        .mount("/copy", routes![copy])
        .mount("/move", routes![movefs])
        .mount("/getconfig", routes![get_config])
        .mount("/getinfo", routes![get_info])
}



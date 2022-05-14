/*
This is the server-side of Clurd built with Rocket.rs
Andrea Canale 2022
Not very beautiful yet.
*/
use rocket::fs::FileServer;
use std::fs;
use std::path::Path;
use std::string::ToString;
use rocket::serde::{Deserialize, json::Json, Serialize};
use json::object;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use fs2;
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self,
        request: &'r Request<'_>,
        response: &mut Response<'r>) {
        response.set_header(Header::new(
            "access-control-allow-origin",
            "*",
        ));
        response.set_header(Header::new(
            "access-control-allow-methods",
            "GET, PATCH, OPTIONS, POST", 
        ));
    }
}
#[macro_use] extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Task<'r> {
    folder: &'r str
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct UploadFile<'r> {
    folder: &'r str,
    name_file: &'r str
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct RenameFile<'r> {
    folder: &'r str,
    new: &'r str
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

struct SpaceFolder { 
    total: String,
    available: String,
 }


 
#[post("/", data = "<file>")]
fn remove(file: Json<Task<'_>>) -> &str{
    let removed = fs::remove_file(file.folder);
    let is_removed = match removed {
        Ok(_removed) => "1",
        Err(_error) => "0",
    };
    is_removed
}

#[post("/", data = "<rename_file>")]
fn rename(rename_file: Json<RenameFile<'_>>) -> &str{
    println!("{}", rename_file.folder);
    let renamed = fs::rename(rename_file.folder, rename_file.new);
    let is_renamed = match renamed {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    is_renamed
}

#[post("/", data = "<rename_file>")]
fn copy(rename_file: Json<RenameFile<'_>>) -> &str{
    let new_path = format!("{}{}", rename_file.new, rename_file.folder);
    let copied = fs::copy(rename_file.folder, new_path);
    let is_copied = match copied {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    is_copied
}

#[post("/", data = "<task>")]
fn space(task: Json<Task<'_>>) -> Json<SpaceFolder> { 
    let pen_space = fs2::free_space(task.folder);
    let u64_space = match pen_space {
        Ok(pen_space) => pen_space,
        Err(_error) => u64::MIN,
    };
    let pen_total = fs2::total_space(task.folder);
    let u64_total = match pen_total {
        Ok(pen_total) => pen_total,
        Err(_error) => u64::MIN,
    };

    Json(SpaceFolder { 
        available: format!("{}", u64_space),
        total: format!("{}", u64_total)
     })
}
#[post("/", data = "<task>")]
fn files(task: Json<Task<'_>>) -> String { 
    println!("{}", task.folder);
    let mut files_raw = json::JsonValue::new_array();

    let paths = fs::read_dir(&Path::new(task.folder)).unwrap();
        
      let names =
      paths.map(|entry| {
        let entry = entry.unwrap();
        
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        
        let file_name_as_str = file_name.to_str().unwrap();
        
        let file_name_as_string = String::from(file_name_as_str);
        
        file_name_as_string
      }).collect::<Vec<String>>();
        for path in names {
            let tpath = format!("{}/{}", task.folder, path);
            let filename = path.clone();
            let bytes_raw = std::fs::read(tpath);
            let bytes = match bytes_raw {
                Ok(bytes_raw) => bytes_raw,
                Err(_error) => Vec::new(),
            };
            let mut hash = sha256::digest_bytes(&bytes);
            if hash == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"{
                hash = String::from("dir");
            }
            files_raw.push(object!{
                file: filename,
                md5: hash,
                absolute: "work in progress"
            }).expect("Error during push of array, open an issue on github");
        }
        files_raw.to_string()
}
#[get("/")]
fn index() ->  &'static str {
    "Welcome to Clurd API"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/getfiles", routes![files])
    .mount("/", routes![index]).attach(Cors)
    .mount("/", FileServer::from("./"))
    .mount("/remove", routes![remove])
    .mount("/space", routes![space])
    .mount("/rename", routes![rename])
    .mount("/copy", routes![copy])
}



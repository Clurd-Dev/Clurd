/*
This is the server-side of Clurd built with Rocket.rs
Andrea Canale 2022
Not very beautiful yet.
*/

use std::fs;
use std::path::Path;
use std::string::ToString;
use rocket::serde::{Deserialize, json::Json};
use json::object;
#[macro_use] extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Task<'r> {
    folder: &'r str
}
#[post("/", data = "<task>")]
fn files(task: Json<Task<'_>>) -> String { 
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
            let filename = path.clone();
            let bytes_raw = std::fs::read(path);
            let bytes = match bytes_raw {
                Ok(bytes_raw) => bytes_raw,
                Err(error) => Vec::new(),
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
    rocket::build().mount("/getfiles", routes![files]).mount("/", routes![index])
}



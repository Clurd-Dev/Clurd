/*
This is the server-side of Clurd built with Rocket.rs
Andrea Canale 2022
Not very beautiful yet.
*/
use fs2;
use json::object;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request, Response};
use std::fs;
use std::path::Path;
use std::string::ToString;
use config_file::FromConfigFile;
use sysinfo::{NetworkExt, System, SystemExt};
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
    let removed = fs::remove_file(file.folder);
    let is_removed = match removed {
        Ok(_removed) => "1",
        Err(_error) => "0",
    };
    is_removed
}

#[post("/", data = "<rename_file>")]
fn rename(rename_file: Json<RenameFile<'_>>) -> &str {
    println!("{}", rename_file.folder);
    let renamed = fs::rename(rename_file.folder, rename_file.new);
    let is_renamed = match renamed {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    is_renamed
}

#[post("/", data = "<rename_file>")]
fn copy(rename_file: Json<RenameFile<'_>>) -> &str {
    let new_path = format!("{}{}", rename_file.new, rename_file.folder);
    let copied = fs::copy(rename_file.folder, new_path);
    let is_copied = match copied {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    is_copied
}
#[post("/", data = "<rename_file>")]
fn movefs(rename_file: Json<RenameFile<'_>>) -> &str {
    let new_path = format!("{}{}", rename_file.new, rename_file.folder);
    let copied = fs::copy(rename_file.folder, new_path);
    let is_copied = match copied {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    let removed = fs::remove_file(rename_file.folder);
    let is_removed = match removed {
        Ok(_removed) => "1",
        Err(_error) => "0",
    };
    if is_copied == "1" && is_removed == "1" {
        "1"
    } else {
        "0"
    }
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
        total: format!("{}", u64_total),
    })
}
#[post("/", data = "<task>")]
fn files(task: Json<Task<'_>>) -> String {
    let mut files_raw = json::JsonValue::new_array();
    let paths = fs::read_dir(&Path::new(task.folder)).unwrap();
    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();

            let file_name_as_str = file_name.to_str().unwrap();

            let file_name_as_string = String::from(file_name_as_str);

            file_name_as_string
        })
        .collect::<Vec<String>>();
    for path in names {
        let hash;
        let mut is_image: bool = false;
        let mut is_video: bool = false;
        let mut is_audio: bool = false;
        let tpath = format!("{}/{}", task.folder, path);
        let filename = tpath.clone();
        let file_json = path.clone();
        let metadata = fs::metadata(filename).expect("Error during files listing.");
        let permission = metadata.permissions().readonly();
        let size = metadata.len();
        let symbolic = metadata.is_symlink();
        if symbolic == true {
        } else {
            let is_dir = metadata.is_dir();
            if is_dir == false {
                let bytes_raw = std::fs::read(tpath);
                let bytes = match bytes_raw {
                    Ok(bytes_raw) => bytes_raw,
                    Err(_error) => Vec::new(),
                };
                hash = sha256::digest_bytes(&bytes);
                is_image = infer::is_image(&bytes);
                is_video = infer::is_video(&bytes);
                is_audio = infer::is_audio(&bytes);
            } else {
                hash = String::from("dir");
            }
            files_raw
                .push(object! {
                    file: file_json,
                    md5: hash,
                    read_only: permission,
                    size: size,
                    image: is_image,
                    video: is_video,
                    audio: is_audio
                })
                .expect("Error during push of array, open an issue on github");
        }
    }

    files_raw.to_string()
}
#[get("/")]
fn index() -> &'static str {
    "Welcome to Clurd API"
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Information { 
    disks: Vec<String>,
    interface: Vec<String>,
    components: Vec<String>,
    total_memory: String,
    used_memory: String,
    total_swap: String,
    used_swap: String,
    system_name: String,
    kernel_version: String,
    system_version: String,
    hostname: String,
    core: String,
    frontend_version: String,
    backend_version: String
 }

#[get("/")]
fn get_info() -> Json<Information>{
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut disks:Vec<String> = Vec::new();
    let mut interface:Vec<String> = Vec::new();
    let mut components:Vec<String> = Vec::new();
    for disk in sys.disks() {
        disks.push(format!("{:?}", disk));
    }
    for (interface_name, data) in sys.networks() {
        interface.push(format!("{}: {}/{} B", interface_name, data.received(), data.transmitted()));
    }
    
    for component in sys.components() {
        components.push(format!("{:?}", component))
    }
    Json(Information { 
        disks: disks,
        interface: interface,
        components: components,
        total_memory: format!("{}", sys.total_memory()),
        used_memory: format!("{}", sys.used_memory()),
        total_swap: format!("{}", sys.total_swap()),
        used_swap: format!("{}", sys.used_swap()),
        system_name: format!("{:?}", sys.name()),
        kernel_version: format!("{:?}", sys.kernel_version()),
        system_version: format!("{:?}", sys.os_version()),
        hostname: format!("{:?}", sys.host_name()),
        core: format!("{}", sys.processors().len()),
        frontend_version: String::from("v1.0"),
        backend_version: String::from("v1.0")
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/getfiles", routes![files])
        .mount("/", routes![index])
        .attach(Cors)
        .mount("/", FileServer::from("./"))
        .mount("/remove", routes![remove])
        .mount("/space", routes![space])
        .mount("/rename", routes![rename])
        .mount("/copy", routes![copy])
        .mount("/move", routes![movefs])
        .mount("/getconfig", routes![get_config])
        .mount("/getinfo", routes![get_info])
}



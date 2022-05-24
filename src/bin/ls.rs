use json::object;
use rocket::serde::{json::Json, Deserialize};

use std::fs;
use std::path::Path;
use std::string::ToString;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Task<'r> {
    folder: &'r str,
}
pub fn list_file(folder: &str) -> String {
    let mut files_raw = json::JsonValue::new_array();
    let paths = fs::read_dir(&Path::new(folder)).unwrap();
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
        let mut is_image: bool = false;
        let mut is_video: bool = false;
        let mut is_audio: bool = false;
        let tpath = format!("{}/{}", folder, path);
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
                is_image = infer::is_image(&bytes);
                is_video = infer::is_video(&bytes);
                is_audio = infer::is_audio(&bytes);
            } else {
            }
            files_raw
                .push(object! {
                    file: file_json,
                    read_only: permission,
                    size: size,
                    image: is_image,
                    video: is_video,
                    audio: is_audio,
                    dir: is_dir
                })
                .expect("Error during push of array, open an issue on github");
        }
    }

    files_raw.to_string()
}

fn main(){
    
}
use fs2;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct Task<'r> {
    folder: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct RenameFile<'r> {
    folder: &'r str,
    new: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

pub struct SpaceFolder {
    total: String,
    available: String,
}

pub fn remove(file: &str) -> &str {
    println!("{}", file);
    let removed = fs::remove_file(file);
    let is_removed = match removed {
        Ok(_removed) => "1",
        Err(_error) => "0",
    };
    is_removed
}

pub fn rename(folder: &str, new: &str) -> String {
    let renamed = fs::rename(folder, new);
    let is_renamed = match renamed {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    String::from(is_renamed)
}

pub fn copy(folder: &str, new: &str) -> String {
    let new_path = format!("{}{}", new, folder);
    let copied = fs::copy(folder, new);
    let is_copied = match copied {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    String::from(is_copied)
}

pub fn movefs(folder: &str, new: &str) -> String {
    let copied = fs::copy(folder, new);
    let is_copied = match copied {
        Ok(_renamed) => "1",
        Err(_error) => "0",
    };
    let removed = fs::remove_file(folder);
    let is_removed = match removed {
        Ok(_removed) => "1",
        Err(_error) => "0",
    };
    if is_copied == "1" && is_removed == "1" {
        String::from("1")
    } else {
        String::from("0")
    }
}



pub fn space(folder: &str) -> Json<SpaceFolder> {
    let pen_space = fs2::free_space(folder);
    let u64_space = match pen_space {
        Ok(pen_space) => pen_space,
        Err(_error) => u64::MIN,
    };
    let pen_total = fs2::total_space(folder);
    let u64_total = match pen_total {
        Ok(pen_total) => pen_total,
        Err(_error) => u64::MIN,
    };

    Json(SpaceFolder {
        available: format!("{}", u64_space),
        total: format!("{}", u64_total),
    })
}

fn main(){

}
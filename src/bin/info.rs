
use rocket::serde::{json::Json, Serialize};
use sysinfo::{NetworkExt, System, SystemExt};
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Information { 
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

pub fn info() -> Json<Information> {
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
        frontend_version: String::from("v1.1"),
        backend_version: String::from("v1.1")
    })
}

fn main(){
    
}
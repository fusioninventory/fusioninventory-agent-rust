use std::{fs::File, io::Read};
use nparse::*;
use std::process::{Command, Stdio};
use std::fs;

pub fn run_inventory() -> serde_json::Value {
    let path: String = std::env::temp_dir().join("dmidecode.txt").display().to_string();
       
    // dmidecode -qt bios -t system > /tmp/dmidecode.txt
    run_dmidecode_cmd(&path);
    let mut out = String::new();
    {
        let mut f = File::open(path.clone()).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.indent_to_json();
    let result = result.unwrap();

    let _ = delete_file(&path);
    return serde_json::json!([
        {
            "key": "manufacturer",
            "value": result["System Information"]["Manufacturer"]
        },
        {
            "key": "chasis",
            "value": result["Chassis Information"]["Type"]
        },
        {
            "key": "serialnumber",
            "value": result["System Information"]["Serial Number"]
        },
        {
            "key": "model",
            "value": result["System Information"]["Product Name"]
        },
        {
            "key": "uuid",
            "value": result["System Information"]["UUID"]
        }
    ]);
}

#[cfg(target_os = "windows")]
fn run_dmidecode_cmd(path: &String) -> bool {
    let file = File::create(path).unwrap();
    let stdio = Stdio::from(file);

    let status = Command::new("dmidecode.exe").arg("-qt").arg("bios").arg("-t").arg("system").arg(">").arg(path).stdout(stdio).status().expect("No such file or directory");

    if status.code() != Some(0) {
        log::error!("dmidecode.exe command not found");
        return false;
    }
    return true;
}

#[cfg(not(target_os = "windows"))]
fn run_dmidecode_cmd(path: &String) -> bool {
    let file = File::create(path).unwrap();
    let stdio = Stdio::from(file);

    let status = Command::new("dmidecode").arg("-qt").arg("chassis").arg("-t").arg("system").arg(">").arg(path).stdout(stdio).status().expect("No such file or directory");

    if status.code() != Some(0) {
        log::error!("dmidecode command not found");
        return false;
    }
    return true;
}

fn delete_file(path: &String) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

#![cfg(target_os = "linux")]
use std::process::{Command, Stdio};

pub fn run_inventory(disk: serde_json::Value) -> Vec<serde_json::Value> {
    log::debug!("[partition] run linux");
    let data = load_lshw_data_json();
    fill_properties(disk, data)
}

fn load_lshw_data_json() -> Vec<serde_json::Value> {
    let cmd = "lshw";
    let args = ["-json", "-class", "volume"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("lshw command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(e) => empty,
    };
    
    let parts: Vec<serde_json::Value> = serde_json::from_str(&data).expect("JSON was not well-formatted in lshw output command");
    return parts;    
}

fn fill_properties(disk: serde_json::Value, data: Vec<serde_json::Value>) -> Vec<serde_json::Value> {

  let mut parts = Vec::new();
  for datapart in data {
    let part = serde_json::json!([
        {
            "key": "id",
            "value": datapart["id"],
        },
        {
            "key": "creationdate",
            "value": "",
        },
        {
            "key": "description",
            "value": datapart["description"],
        },
        {
            "key": "size",
            "type": "integer",
            "unit": "B",
            "value": datapart["size"],
        },
        {
            "key": "usedsize",
            "value": "",
        },
        {
            "key": "freesize",
            "value": "",
        },
        {
            "key": "filesystem",
            "value": "",
        },
        {
            "key": "label",
            "value": datapart["logicalname"],
        },
        {
            "key": "mountpoint",
            "value": "",
        },
        {
            "key": "serial",
            "value": datapart["serial"],
        },
        {
            "key": "system",
            "value": "",
        },
        {
            "key": "ostype",
            "value": "",
        },
        {
            "key": "encryption",
            "value": "",
        },
        {
            "key": "algorithm",
            "value": "",
        },
        {
            "key": "encryptedstatus",
            "value": "",
        },
        {
            "key": "encryptedtype",
            "value": "",
        }
    ]);
    parts.push(part);
  }
  return parts;
}

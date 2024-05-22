use std::{fs::File, io::Read};
use nparse::*;
use std::process::{Command, Stdio};
use std::fs;

pub fn run_inventory() -> serde_json::Value {
    let path: String = std::env::temp_dir().join("dmidecode.txt").display().to_string();
    
    let mut cachel1 = serde_json::json!(null);
    let mut cachel2 = serde_json::json!(null);
    let mut cachel3 = serde_json::json!(null);

    run_dmidecode_cmd(&path);
    let mut out = String::new();
    {
        let mut f = File::open(path.clone()).unwrap();
        f.read_to_string(&mut out).unwrap();
    }

    let parts = out.split("\n\n");
    for part in parts {
        let result = part.to_string().indent_to_json();
        let result = result.unwrap();

        if result["Cache Information"]["Socket Designation"] == "L1 - Cache" {
            cachel1 = result["Cache Information"]["Installed Size"].clone();
        } else if result["Cache Information"]["Socket Designation"] == "L2 - Cache" {
            cachel2 = result["Cache Information"]["Installed Size"].clone();
        } else if result["Cache Information"]["Socket Designation"] == "L3 - Cache" {
            cachel3 = result["Cache Information"]["Installed Size"].clone();
        }
    }

    // load all to have processor
    let result = out.indent_to_json();
    let result = result.unwrap();

    let _ = delete_file(&path);
    return serde_json::json!([
        {
            "key": "manufacturer",
            "value": result["Processor Information"]["Manufacturer"]
        },
        {
            "key": "id",
            "value": result["Processor Information"]["ID"]
        },
        {
            "key": "family",
            "value": result["Processor Information"]["Family"]
        },
        {
            "key": "speed",
            "type": "integer",
            "unit": "Mhz",
            "value": result["Processor Information"]["Max Speed"]
        },
        {
            "key": "serialnumber",
            "value": result["Processor Information"]["Serial Number"]
        },
        {
            "key": "corecount",
            "type": "integer",
            "value": result["Processor Information"]["Core Count"]
        },
        {
            "key": "threadcount",
            "type": "integer",
            "value": result["Processor Information"]["Thread Count"]
        },
        {
            "key": "flags",
            "type": "list",
            "value": [] // TODO
        },
        {
            "key": "characteristics",
            "type": "list",
            "value": [] // TODO
        },
        {
            "key": "L1 cache",
            "type": "integer",
            "value": cachel1
        },
        {
            "key": "L2 cache",
            "type": "integer",
            "value": cachel2
        },
        {
            "key": "L2 cache",
            "type": "integer",
            "value": cachel3
        }
    ]);
}

#[cfg(target_os = "windows")]
fn run_dmidecode_cmd(path: String) -> bool {
    let file = File::create(path).unwrap();
    let stdio = Stdio::from(file);

    let status = Command::new("dmidecode.exe").arg("-qt").arg("processor").arg(">").arg(path).stdout(stdio).status().expect("No such file or directory");

    if status.code() != 0 {
        log::error!("dmidecode.exe command not found");
        return false;
    }
    return true;
}

#[cfg(not(target_os = "windows"))]
fn run_dmidecode_cmd(path: &String) -> bool {
    let file = File::create(path).unwrap();
    let stdio = Stdio::from(file);

    let cmd = "dmidecode";
    let args = ["-qt", "processor", "-t", "cache"];
    let output = Command::new(cmd).args(&args).output();
    let output = output.unwrap();
    let output = String::from_utf8_lossy(&output.stdout[..]).to_string();

    let args = ["-qt", "processor", "-t", "cache"];

    let status = Command::new("dmidecode").args(&args).arg(">").arg(path).stdout(stdio).status().expect("No such file or directory");

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

// Copyright (C) 2024 FusionSuite Team
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details. You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see <https://www.gnu.org/licenses/>.

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

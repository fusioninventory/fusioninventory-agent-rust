#![cfg(target_os = "linux")]
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

use std::process::{Command, Stdio};

pub fn run_inventory() -> Vec<serde_json::Value> {
    let data = load_lshw_data_json();
    fill_properties(data)
}

fn load_lshw_data_json() -> Vec<serde_json::Value> {
    let cmd = "lshw";
    let args = ["-json", "-class", "disk"];

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
    
    let disks: Vec<serde_json::Value> = serde_json::from_str(&data).expect("JSON was not well-formatted in lshw output command");
    return disks;    
}

fn fill_properties(data: Vec<serde_json::Value>) -> Vec<serde_json::Value> {

  let mut disks = Vec::new();
  for datadisk in data {
    let disk = serde_json::json!([
        {
            "key": "name",
            "value": datadisk["product"],
        },
        {
            "key": "description",
            "value": datadisk["description"],
        },
        {
            "key": "serialnumber",
            "value": datadisk["serial"],
        },
        {
            "key": "size",
            "type": "integer",
            "unit": "B",
            "value": 0,
        },
        {
            "key": "id",
            "value": datadisk["id"],
        }
    ]);
    disks.push(disk);
  }
  return disks;
}

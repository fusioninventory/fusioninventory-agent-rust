#![cfg(target_os = "freebsd")]
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
use regex::Regex;
use std::collections::HashMap;

pub fn run_inventory() -> Vec<serde_json::Value> {
    let filesystems = get_mounted();
    fill_properties(filesystems)
}

fn get_mounted() -> Vec<HashMap<String, String>> {

    let cmd = "mount";
    let args = ["-v"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("mount -v command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(_) => empty,
    };
    parse_output(data)
}

fn parse_output(data: String) -> Vec<HashMap<String, String>> {
    let mut filesystems: Vec<HashMap<String, String>> = Vec::new();

    let re: Regex = Regex::new(r"^(\/dev[\w\/]+) on ([\w\/]+) \((\w+)").unwrap();
    for line in data.lines() {
        if let Some(mat) = re.captures(line) {
            let mut fs_attr: HashMap<String, String> = HashMap::new();
            fs_attr.insert(String::from("name"), mat[2].trim().to_string());
            fs_attr.insert(String::from("partition"), mat[1].trim().to_string());
            fs_attr.insert(String::from("type"), mat[3].trim().to_string());
            filesystems.push(fs_attr);
        }
    }
    return filesystems;
}

fn fill_properties(filesystems: Vec<HashMap<String, String>>) -> Vec<serde_json::Value> {
    let mut filesystems_prop = Vec::new();

    for fs in filesystems.iter() {
        filesystems_prop.push(serde_json::json!([
            {
                "key": "type",
                "value": fs.get("type")
            },
            {
                "key": "name",
                "value": fs.get("name")
            },
            {
                "key": "partition",
                "value": fs.get("partition")
            }
        ]));
    }

    return filesystems_prop;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/filesystem/freebsd_mount.dat").unwrap();
        let result: Vec<HashMap<String, String>> = parse_output(data);
        let mut expected: Vec<HashMap<String, String>> = Vec::new();
        expected.push(HashMap::from([
            ("type".to_string(), "msdosfs".to_string()),
            ("name".to_string(), "/boot/efi".to_string()),
            ("partition".to_string(), "/dev/gpt/efiboot0".to_string()),
        ]));
        assert_eq!(result, expected);
    }
}

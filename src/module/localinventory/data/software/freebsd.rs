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
// use chrono::prelude::{DateTime, Utc};

pub fn run_inventory() -> Vec<serde_json::Value> {

    let packages = load_from_pkg();
    fill_properties(packages)
}

fn load_from_pkg() -> Vec<serde_json::Value> {

    let mut packages = Vec::new();
    let cmd = "pkg";
    let args = ["info", "--raw", "--raw-format", "json-compact", "--all"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("pkg command error");

    // split
    let empty = String::from("");
    let linesstr = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(e) => empty,
    };

    let lines = linesstr.lines();
    for line in lines {
        if line != "" {
            let json_test: serde_json::Value = serde_json::from_str(line).expect("JSON was not well-formatted line by line");
            packages.push(json_test);
        }
    }

    // TODO split each line and read json
    // serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).expect("JSON was not well-formatted")
    // serde_json::from_slice(output.stdout.as_slice()).expect("JSON was not well-formatted")

    return packages;    
}

fn fill_properties(packages: Vec<serde_json::Value>) -> Vec<serde_json::Value> {

    let mut softwares = Vec::new();
    for package in packages {
        // let installationdate: DateTime<Utc> = package["timestamp"];
        let data = serde_json::json!([
            {
                "key": "cleanedname",
                "value": package["name"],
            },
            {
                "key": "version",
                "value": package["version"],
            },
            {
                "key": "publisher",
                "value": "",
            },
            {
                "key": "maintainer",
                "value": package["maintainer"],
            },
            {
                "key": "source",
                "value": package["annotations"]["repository"],
            },
            {
                "key": "type",
                "value": "",
            },
            {
                "key": "architecture",
                "value": package["arch"],
            },
            {
                "key": "category",
                "value": package["categories"][0],
            },
            {
                "key": "installationdate",
                "value": "", // installationdate.format("%Y-%m-%d").to_string(),
            },
            {
                "key": "uninstallcommand",
                "value": String::from("pkg delete -f").push_str(&package["name"].to_string()),
            },
            {
                "key": "guid",
                "value": "",
            },
            {
                "key": "comment",
                "value": package["comment"],
            },
            {
                "key": "mainurl",
                "value": package["www"],
            },
            {
                "key": "helpurl",
                "value": "",
            }
        ]);
        softwares.push(data);
    }
    return softwares;
}

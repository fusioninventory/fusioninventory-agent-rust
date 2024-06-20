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

use crate::module;

use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use module::localinventory::structure::software::FinalPackageStruct;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
struct PackageStruct {
    name: String,
    version: String,
    maintainer: String,
    arch: String,
    comment: String,
    mainurl: String,
    helpurl: String,
    timestamp: u64,
    publisher: String,
    guid: String,
}

pub fn run_inventory() -> Vec<FinalPackageStruct> {
    let output_pacman = get_pacman();
    parse_pacman_output(output_pacman)
}

fn get_pacman() -> String {
    let args = [
        "-Qqi"
    ];
    match Command::new("pacman")
        .args(args)
        .stdout(Stdio::piped())
        .output() {
            Ok(output) => {
                let empty = String::from("");
                match String::from_utf8(output.stdout) {
                    Ok(x) => x,
                    Err(_) => empty,
                }
            },
            Err(_) => {
                println!("pacman command error");
                "".to_string()
            }
        }
}

fn parse_pacman_output(output: String) -> Vec<FinalPackageStruct> {
    let mut softwares: Vec<FinalPackageStruct> = Vec::new();
    // split each line (each line is a software)
    let parts = output.split("\n");
    for part in parts {
        // split 
        let pp: String = part.to_string();
        // let mut software_info = pp.split("\t\t");
        let software_info: Vec<&str> = pp.split("\t\t").collect();
        // let mut soft = HashMap::new();
        if software_info.iter().count() < 11 {
            continue;
        }
        // let mut installdate = "".to_string();
        // match software_info[4].to_string().clone().parse::<u64>() {
        //     Ok(t) => {
        //         let d = UNIX_EPOCH + Duration::from_secs(t);
        //         installdate = DateTime::<Utc>::from(d).format("%Y-%m-%d").to_string();
        //     },
        //     Err(e) => println!("err on timestamp {:?}", e)
        // }

        let my_software: FinalPackageStruct = FinalPackageStruct {
            name: "".to_string(), // software_info[0].to_string().clone(),
            originalname: "".to_string(),
            version: "".to_string(),
            revision: "".to_string(),
            originalversion: "".to_string(),
            publisher: "".to_string(),
            maintainer: "".to_string(),
            repository: "".to_string(),
            pkgtype: "pacman".to_string(),
            architecture: "".to_string(),
            category: "".to_string(),
            installationdate: "".to_string(),
            uninstallcommand: "".to_string(),
            id: "".to_string(),
            comment: "".to_string(),
            mainurl: "".to_string(),
            helpurl: "".to_string(),
        };
        softwares.push(my_software);
    }
    return softwares;
}

#[cfg(test)]
mod tests {
    use super::*;
}

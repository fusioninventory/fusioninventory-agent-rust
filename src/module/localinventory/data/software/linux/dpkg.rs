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
    let output_apt = get_dpkg();
    parse_dpkg_output(output_apt)
}

fn get_dpkg() -> String {
    // https://crates.io/crates/dpkg-query-json
    let args = [
        "--show",
        "--showformat='${Package}\t\t${Architecture}\t\t${Version}\t\t${Installed-Size}\t\t${Section}\t\t${Status}\t\t${Homepage}\t\t${Maintainer}\t\t${binary:Summary}\n"
    ];
    match Command::new("dpkg-query")
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
            Err(e) => {
                println!("dpkg command error: {:?}", e);
                "".to_string()
            }
        }
}

fn parse_dpkg_output(output: String) -> Vec<FinalPackageStruct> {
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

        let my_software: FinalPackageStruct = FinalPackageStruct {
            name: software_info[0].to_string().clone(),
            originalname: software_info[0].to_string().clone(),
            version: software_info[2].to_string().clone(),
            revision: "".to_string(),
            originalversion: software_info[2].to_string().clone(),
            publisher: "".to_string(),
            maintainer: software_info[7].to_string().clone(),
            repository: "".to_string(),
            pkgtype: "".to_string(),
            architecture: software_info[1].to_string().clone(),
            category: "".to_string(),
            installationdate: "".to_string(),
            uninstallcommand: "".to_string(),
            id: "".to_string(),
            comment: software_info[8].to_string().clone(),
            mainurl: software_info[6].to_string().clone(),
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

// #![cfg(target_os = "linux")]
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
use regex::Regex;

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
    let output_snap = get_snap_list();
    // parse_snap_package("losslesscut".to_string())
    let data: Vec<FinalPackageStruct> = Vec::new();
    return data;
}

fn get_snap_list() -> String {
    let args = [
        "list",
        "--color",
        "never"
    ];
    match Command::new("snap")
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
                println!("snap list command error");
                "".to_string()
            }
        }
}

fn get_snap_package(packagename: String) -> String {
    let args = [
        "info",
        "--color",
        "never",
        "--abs-time",
        packagename.as_str()
    ];
    match Command::new("snap")
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
                println!("snap info command error");
                "".to_string()
            }
        }

}

fn parse_snap_extract_packagename(output: String) -> Vec<String> {
    let mut names = Vec::new();

    let re: Regex = Regex::new(r"^(\S*)").unwrap();
    for line in output.lines() {
        if let Some(mat) = re.captures(line) {
            if mat[1].trim().to_string() == "Name" {
                continue;
            }
            names.push(mat[1].trim().to_string());
        }
    }
    return names;
}

fn parse_snap_package(output: String) -> FinalPackageStruct {
    // TODO
    let mut softwares: Vec<FinalPackageStruct> = Vec::new();
    // split each line (each line is a software)
    let parts = output.split("\n");
    let mut name = "".to_string();
    let mut comment = "".to_string();
    let mut publisher = "".to_string();
    let mut mainurl = "".to_string();
    let mut id = "".to_string();
    let mut version = "".to_string();

    let re: Regex = Regex::new(r"^(name|summary|publisher|store-url|snap-id)\:(?:\s*)([\S ]*)").unwrap();
    let re_installed: Regex = Regex::new(r"^installed\:(?:\s*)([\S]*)").unwrap();

    for part in parts {
        // split 
        let pp: String = part.to_string();
        // let mut software_info = pp.split("\t\t");
        for line in output.lines() {
            if let Some(mat) = re.captures(line) {
                if mat[1].trim().to_string() == "name" {
                    name = mat[2].trim().to_string();
                }
                if mat[1].trim().to_string() == "summary" {
                    comment = mat[2].trim().to_string();
                }
                if mat[1].trim().to_string() == "publisher" {
                    publisher = mat[2].trim().to_string();
                }
                if mat[1].trim().to_string() == "store-url" {
                    mainurl = mat[2].trim().to_string();
                }
                if mat[1].trim().to_string() == "snap-id" {
                    id = mat[2].trim().to_string();
                }
            } else if let Some(mat) = re_installed.captures(line) {
                version = mat[1].trim().to_string();
            }
        }    
    }
    FinalPackageStruct {
        name: module::localinventory::data::software::common::clean_name(name.clone()),
        originalname: name,
        version: version.clone(),
        revision: "".to_string(),
        originalversion: version,
        publisher: publisher,
        maintainer: "".to_string(),
        repository: "".to_string(),
        pkgtype: "snap".to_string(),
        architecture: "".to_string(),
        category: "".to_string(),
        installationdate: "".to_string(),
        uninstallcommand: "".to_string(),
        id: id,
        comment: comment,
        mainurl: mainurl,
        helpurl: "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snap_extract_packagename() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/software/snaplist.dat").unwrap();
        let result: Vec<String> = parse_snap_extract_packagename(data);
        let mut expected: Vec<String> = Vec::new();
        expected.push("bare".to_string());
        expected.push("core18".to_string());
        expected.push("gnome-3-28-1804".to_string());
        expected.push("gtk-common-themes".to_string());
        expected.push("losslesscut".to_string());
        expected.push("snapd".to_string());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_snap_package() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/software/snap_losslesscut.dat").unwrap();
        let result: FinalPackageStruct = parse_snap_package(data);
        let expected: FinalPackageStruct = serde_json::from_value(serde_json::json!({
            "name": "losslesscut".to_string(),
            "originalname": "losslesscut".to_string(),
            "version": "3.60.0".to_string(),
            "revision": "".to_string(),
            "originalversion": "3.60.0".to_string(),
            "publisher": "Mikael Finstad (mifino)".to_string(),
            "maintainer": "".to_string(),
            "repository": "".to_string(),
            "pkgtype": "snap".to_string(),
            "architecture": "".to_string(),
            "category": "".to_string(),
            "installationdate": "".to_string(),
            "uninstallcommand": "".to_string(),
            "id": "uRYH61YLB0C3jpYE5sUqf3Tr3KSfuILe".to_string(),
            "comment": "The swiss army knife of lossless video/audio editing".to_string(),
            "mainurl": "https://snapcraft.io/losslesscut".to_string(),
            "helpurl": "".to_string(),
        })).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_snap_package_gnome() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/software/snap_gnome-3-28-1804.dat").unwrap();
        let result: FinalPackageStruct = parse_snap_package(data);
        let expected: FinalPackageStruct = serde_json::from_value(serde_json::json!({
            "name": "gnome".to_string(),
            "originalname": "gnome-3-28-1804".to_string(),
            "version": "3.28.0-19-g98f9e67.98f9e67".to_string(),
            "revision": "".to_string(),
            "originalversion": "3.28.0-19-g98f9e67.98f9e67".to_string(),
            "publisher": "Canonical**".to_string(),
            "maintainer": "".to_string(),
            "repository": "".to_string(),
            "pkgtype": "snap".to_string(),
            "architecture": "".to_string(),
            "category": "".to_string(),
            "installationdate": "".to_string(),
            "uninstallcommand": "".to_string(),
            "id": "TKv5Fm000l4XiUYJW9pjWHLkCPlDbIg1".to_string(),
            "comment": "Shared GNOME 3.28 runtime for Ubuntu 18.04".to_string(),
            "mainurl": "https://snapcraft.io/gnome-3-28-1804".to_string(),
            "helpurl": "".to_string(),
        })).unwrap();

        assert_eq!(result, expected);
    }

}

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

use crate::module;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use chrono::prelude::DateTime;
use chrono::Utc;
use serde_json::error::Category;
use std::time::{UNIX_EPOCH, Duration};
use module::localinventory::structure::software::FinalPackageStruct;


#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
struct AnnotationsStruct {
    repository: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
struct PackageStruct {
    name: String,
    version: String,
    maintainer: String,
    arch: String,
    comment: String,
    www: String,
    annotations: Option<AnnotationsStruct>,
    categories: Vec<String>,
    timestamp: u64,
}

pub fn run_inventory() -> Vec<FinalPackageStruct> {

    let packages = load_from_pkg();
    fill_properties(packages)
}

fn load_from_pkg() -> Vec<PackageStruct> {

    let cmd = "pkg";
    let args = ["info", "--raw", "--raw-format", "json-compact", "--all"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("pkg command error");

    // split
    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(_) => empty,
    };
    parse_output(data)
}

fn parse_output(data: String) -> Vec<PackageStruct> {
    let mut packages: Vec<PackageStruct> = Vec::new();

    for line in data.lines() {
        if line != "" {
            // TODO on some versions, not have line by line, so split by }{
            let parts = line.split("}{");
            let cnt = parts.clone().count();
            if cnt == 1 {
                let pkg_software: PackageStruct = serde_json::from_str::<PackageStruct>(line).expect("JSON was not well-formatted line by line");
                packages.push(pkg_software);
    
            } else if cnt > 1 {
                let mut index = 0;
                for part in parts {
                    index = index + 1;
                    let mut newline;
                    if index == 1 {
                        newline = format!("{}{}", part, "}");
                    } else if index == cnt {
                        newline = format!("{}{}", "{", part);
                    } else {
                        newline = format!("{}{}{}", "{", part, "}");
                    }


                    let pkg_software: PackageStruct = serde_json::from_str::<PackageStruct>(newline.as_str()).expect("JSON was not well-formatted line by line");
                    packages.push(pkg_software);
                    
                }
            }
        }
    }
    return packages;    
}

fn fill_properties(packages: Vec<PackageStruct>) -> Vec<FinalPackageStruct> {

    let mut softwares = Vec::new();
    for package in packages {
        // let installationdate: DateTime<Utc> = package["timestamp"];
        let mut repository = "".to_string();
        match package.annotations {
            Some(x) => {
                match x.repository {
                    Some(y) => repository = y,
                    None    => repository = "".to_string(),
                }
            },
            None    => repository = "".to_string(),
        }
        let d = UNIX_EPOCH + Duration::from_secs(package.timestamp);
        let installdate = DateTime::<Utc>::from(d);
        let mut uninstallcommand = String::from("pkg delete -f ");
        uninstallcommand.push_str(package.name.as_str());
        let data: FinalPackageStruct = FinalPackageStruct {
            name: package.name.clone(),
            originalname: package.name,
            version: package.version.clone(),
            revision: "".to_string(),
            originalversion: package.version,
            publisher: "".to_string(),
            maintainer: package.maintainer,
            repository: repository,
            pkgtype: "pkg".to_string(),
            architecture: package.arch,
            category: "".to_string(),
            installationdate: installdate.format("%Y-%m-%d").to_string(),
            uninstallcommand: uninstallcommand,
            id: "".to_string(),
            comment: package.comment,
            mainurl: package.www,
            helpurl: "".to_string(),
        };
        softwares.push(data);
    }
    return softwares;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/software/freebsd_pkg.dat").unwrap();
        // load data json of pkg from file because directly here is difficult
        let result: Vec<PackageStruct> = parse_output(data);
        let mut expected: Vec<PackageStruct> = Vec::new();
        expected.push(serde_json::from_value(serde_json::json!({
            "name": "Box2D",
            "version": "2.4.1",
            "maintainer": "yuri@FreeBSD.org",
            "arch": "freebsd:14:x86:64",
            "comment": "2D physics engine for games",
            "www": "https://box2d.org/",
            "annotations": {
                "repository": "durieux_family_poudriere"
            },
            "categories": ["misc"],
            "timestamp": 1709152837,
        })).unwrap());

        expected.push(serde_json::from_value(serde_json::json!({
            "name": "GentiumBasic",
            "version": "1102_1",
            "maintainer": "matthew@FreeBSD.org",
            "arch": "freebsd:14:*",
            "comment": "Gentium Basic and Gentium Book Basic TrueType fonts",
            "www": "https://software.sil.org/gentium/",
            "annotations": {
                "repository": "durieux_family_poudriere"
            },
            "categories": ["x11-fonts"],
            "timestamp": 1709152837,
        })).unwrap());

        expected.push(serde_json::from_value(serde_json::json!({
            "name": "Imath",
            "version": "3.1.10",
            "maintainer": "mandree@FreeBSD.org",
            "arch": "freebsd:14:x86:64",
            "comment": "C++/Python lib of 2D/3D vector, matrix, math ops for computer graphics",
            "www": "https://github.com/AcademySoftwareFoundation/Imath/",
            "annotations": {
                "repository": "durieux_family_poudriere"
            },
            "categories": ["math","graphics","devel"],
            "timestamp": 1709152565,
        })).unwrap());

        expected.push(serde_json::from_value(serde_json::json!({
            "name": "aalib",
            "version": "1.4.r5_14",
            "maintainer": "ports@FreeBSD.org",
            "arch": "freebsd:14:x86:64",
            "comment": "ASCII art library",
            "www": "https://aa-project.sourceforge.net/aalib/",
            "annotations": {
                "repository": "durieux_family_poudriere"
            },
            "categories": ["graphics"],
            "timestamp": 1710580344,
        })).unwrap());

        expected.push(serde_json::from_value(serde_json::json!({
            "name": "bsddialog",
            "version": "1.0.1",
            "maintainer": "alfix86@gmail.com",
            "arch": "freebsd:14:x86:64",
            "comment": "Text User Interface Widgets",
            "www": "https://gitlab.com/alfix/bsddialog",
            "annotations": {
                "repository": null
            },
            "categories": ["devel"],
            "timestamp": 1714125640,
        })).unwrap());
 
        assert_eq!(result, expected);
    
    }
}

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
    let output_rpm = get_rpm();
    parse_rpm_output(output_rpm)
}

fn get_rpm() -> String {
    let args = [
        "-qa",
        "--queryformat",
        "%{NAME}\t\t%{ARCH}\t\t%{VERSION}\t\t%{RELEASE}\t\t%{INSTALLTIME}\t\t%{VENDOR}\t\t%{SUMMARY}\t\t%{GROUP}\t\t%{PACKAGER}\t\t%{URL}\t\t%{BUGURL}\t\t%{PKGID}\n"
    ];
    match Command::new("rpm")
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
                println!("rpm command error");
                "".to_string()
            }
        }
}

fn parse_rpm_output(output: String) -> Vec<FinalPackageStruct> {
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
        let mut installdate = "".to_string();
        match software_info[4].to_string().clone().parse::<u64>() {
            Ok(t) => {
                let d = UNIX_EPOCH + Duration::from_secs(t);
                installdate = DateTime::<Utc>::from(d).format("%Y-%m-%d").to_string();
            },
            Err(e) => println!("err on timestamp {:?}", e)
        }

        let my_software: FinalPackageStruct = FinalPackageStruct {
            name: software_info[0].to_string().clone(),
            originalname: software_info[0].to_string().clone(),
            version: software_info[2].to_string().clone(),
            revision: software_info[3].to_string().clone(),
            originalversion: software_info[2].to_string().clone(),
            publisher: "".to_string(),
            maintainer: software_info[8].to_string().clone(),
            repository: software_info[5].to_string().clone(),
            pkgtype: "rpm".to_string(),
            architecture: software_info[1].to_string().clone(),
            category: "".to_string(),
            installationdate: installdate,
            uninstallcommand: "".to_string(),
            id: software_info[11].to_string().clone(),
            comment: software_info[6].to_string().clone(),
            mainurl: software_info[9].to_string().clone(),
            helpurl: software_info[10].to_string().clone(),
        };
        softwares.push(my_software);
    }
    return softwares;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rpm_output() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/software/fedora_rpm.dat").unwrap();
        let result: Vec<FinalPackageStruct> = parse_rpm_output(data);
        let mut expected: Vec<FinalPackageStruct> = Vec::new();

        expected.push(FinalPackageStruct {
            name: "libgcc".to_string(),
            originalname: "libgcc".to_string(),
            version: "14.1.1".to_string(),
            revision: "5.fc40".to_string(),
            originalversion: "14.1.1".to_string(),
            publisher: "".to_string(),
            maintainer: "Fedora Project".to_string(),
            repository: "Fedora Project".to_string(),
            pkgtype: "rpm".to_string(),
            architecture: "x86_64".to_string(),
            category: "".to_string(),
            installationdate: "2024-06-15".to_string(),
            uninstallcommand: "".to_string(),
            id: "499b0b8f20a953dfe935e85bd93a1cb2".to_string(),
            comment: "GCC version 14 shared support library".to_string(),
            mainurl: "http://gcc.gnu.org".to_string(),
            helpurl: "https://bugz.fedoraproject.org/gcc".to_string(),
        });

        expected.push(FinalPackageStruct {
            name: "fonts-filesystem".to_string(),
            originalname: "fonts-filesystem".to_string(),
            version: "2.0.5".to_string(),
            revision: "14.fc40".to_string(),
            originalversion: "2.0.5".to_string(),
            publisher: "".to_string(),
            maintainer: "Fedora Project".to_string(),
            repository: "Fedora Project".to_string(),
            pkgtype: "rpm".to_string(),
            architecture: "noarch".to_string(),
            category: "".to_string(),
            installationdate: "2024-06-15".to_string(),
            uninstallcommand: "".to_string(),
            id: "468b8f3f8c2d1e4c10f0c3f4f4e736c3".to_string(),
            comment: "Directories used by font packages".to_string(),
            mainurl: "https://docs.fedoraproject.org/en-US/packaging-guidelines/FontsPolicy/".to_string(),
            helpurl: "https://bugz.fedoraproject.org/fonts-rpm-macros".to_string(),
        });

        expected.push(FinalPackageStruct {
            name: "xkeyboard-config".to_string(),
            originalname: "xkeyboard-config".to_string(),
            version: "2.41".to_string(),
            revision: "1.fc40".to_string(),
            originalversion: "2.41".to_string(),
            publisher: "".to_string(),
            maintainer: "Fedora Project".to_string(),
            repository: "Fedora Project".to_string(),
            pkgtype: "rpm".to_string(),
            architecture: "noarch".to_string(),
            category: "".to_string(),
            installationdate: "2024-06-15".to_string(),
            uninstallcommand: "".to_string(),
            id: "2935adee5a077f929fceb628bd4cb113".to_string(),
            comment: "X Keyboard Extension configuration data".to_string(),
            mainurl: "http://www.freedesktop.org/wiki/Software/XKeyboardConfig".to_string(),
            helpurl: "https://bugz.fedoraproject.org/xkeyboard-config".to_string(),
        });

        assert_eq!(result, expected);
    }
}

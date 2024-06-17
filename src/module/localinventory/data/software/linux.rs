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
use std::collections::HashMap;

pub fn run_inventory() -> Vec<serde_json::Value> {
    // https://github.com/sigoden/upt

    load_from_package_manager()
}

fn load_from_package_manager() -> Vec<serde_json::Value> {
    // let data = get_dpkg();
    // println!("dpkg return: {:?}", data);
    let output = get_rpm();
    // println!("rpm return: {:?}", output);
    let data = parse_rpm_output(output);

    fill_properties(data)
}

fn get_dpkg() -> Option<String> {
    // https://crates.io/crates/dpkg-query-json
    let args = [
        "--show",
        "--showformat='${Package}\t\t${Architecture}\t\t${Version}\t\t${Installed-Size}\t\t${Section}\t\t${Status}\t\t${Homepage}\t\t${Maintainer}\t\t${binary:Summary}\n"
    ];
    let output = match Command::new("dpkg-query")
        .args(args)
        .stdout(Stdio::piped())
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                match String::from_utf8(output.stdout) {
                    Ok(x) => {
                        return Some(x);
                    },
                    Err(_) => "".to_string(),
                }
            } else {
                return Some("".to_string());
            }
        },
        _ => {
            log::debug!("apt not found, next package manager");
            return Some("".to_string());
        },
    };
    None
}

fn get_rpm() -> String {
    let args = [
        "-qa",
        "--queryformat",
        "%{NAME}\t\t%{ARCH}\t\t%{VERSION}-%{RELEASE}\t\t%{INSTALLTIME}\t\t%{VENDOR}\t\t%{SUMMARY}\t\t%{GROUP}\t\t%{PACKAGER}\t\t%{URL}\t\t%{BUGURL}\t\t%{PKGID}\n"
    ];
    let output = Command::new("rpm")
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .expect("rpm command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(e) => empty,
    };

    return data;
}

fn parse_rpm_output(output: String) -> Vec<HashMap<String, String>> {
    let mut softwares = Vec::new();
    // split each line (each line is a software)
    let parts = output.split("\n");
    for part in parts {
        // split 
        let pp: String = part.to_string();
        // let mut software_info = pp.split("\t\t");
        let software_info: Vec<&str> = pp.split("\t\t").collect();
        let mut soft = HashMap::new();
        if software_info.iter().count() < 11 {
            continue;
        }

        soft.insert(String::from("name"), software_info[0].to_string().clone());
        soft.insert(String::from("arch"), software_info[1].to_string().clone());
        soft.insert(String::from("version"), software_info[2].to_string().clone());
        soft.insert(String::from("installationdate"), software_info[3].to_string().clone());
        soft.insert(String::from("publisher"), software_info[4].to_string().clone());
        soft.insert(String::from("comment"), software_info[5].to_string().clone());
        soft.insert(String::from("maintainer"), software_info[7].to_string().clone());
        soft.insert(String::from("mainurl"), software_info[8].to_string().clone());
        soft.insert(String::from("helpurl"), software_info[9].to_string().clone());
        // it''s more id than uuid
        soft.insert(String::from("guid"), software_info[10].to_string().clone());
        softwares.push(soft);
    }
    return softwares;
}

fn fill_properties(packages: Vec<HashMap<String, String>>) -> Vec<serde_json::Value> {

    let mut softwares = Vec::new();
    for package in packages {
        // let installationdate: DateTime<Utc> = package["timestamp"];
        let data = serde_json::json!([
            {
                "key": "cleanedname",
                "value": package.get("name"),
            },
            {
                "key": "version",
                "value": package.get("version"),
            },
            {
                "key": "publisher",
                "value": package.get("publisher"),
            },
            {
                "key": "maintainer",
                "value": package.get("maintainer"),
            },
            {
                "key": "source",
                "value": "",
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
                "value": "",
            },
            {
                "key": "installationdate",
                "value": package.get("installationdate"), // .format("%Y-%m-%d").to_string(), // installationdate.format("%Y-%m-%d").to_string(),
            },
            {
                "key": "uninstallcommand",
                "value": "",
            },
            {
                "key": "guid",
                "value": package.get("guid"),
            },
            {
                "key": "comment",
                "value": package["comment"],
            },
            {
                "key": "mainurl",
                "value": package["mainurl"],
            },
            {
                "key": "helpurl",
                "value": package["helpurl"],
            }
        ]);
        softwares.push(data);
    }
    return softwares;
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_rpm_output() {
        // load data from file tests/software_rpm.data

        // send it in parser function


        // verify content converted

    
    }
}
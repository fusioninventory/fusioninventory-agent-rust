#![cfg(target_os = "linux")]
use std::process::{Command, Stdio};
use std::collections::HashMap;

pub fn run_inventory() {
    // https://github.com/sigoden/upt

    load_from_package_manager();
    // fill_properties(packages);
}

fn load_from_package_manager() {
    let data = get_dpkg();
    println!("dpkg return: {:?}", data);
    let data = get_rpm();
    println!("rpm return: {:?}", data);
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

fn get_rpm() -> Option<String> {
    let args = [
        "-qa",
        "--queryformat",
        "'%{NAME}\t\t%{ARCH}\t\t%{VERSION}-%{RELEASE}\t\t%{INSTALLTIME}\t\t%{VENDOR}\t\t%{SUMMARY}\t\t%{GROUP}\t\t%{PACKAGER}\t\t%{URL}\t\t%{BUGURL}\t\t%{PKGID}\n'"
    ];
    let output = match Command::new("rpm")
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
            log::debug!("rpm not found, next package manager");
            return Some("".to_string());
        },
    };
    None
}

fn parse_rpm_output(output: String) -> Vec<Vec<String>> {
    let softwares = Vec::new();
    // split each line (each line is a software)
    let parts = output.split("\n");
    for part in parts {
        // split 
        let software_info = part.split("\t\t");
        let mut soft = HashMap::new();

        soft.insert(String::from("name"), software_info[0]);
        soft.insert(String::from("arch"), software_info[1]);
        soft.insert(String::from("version"), software_info[2]);
        soft.insert(String::from("installationdate"), software_info[3]);
        soft.insert(String::from("publisher"), software_info[4]);
        soft.insert(String::from("comment"), software_info[5]);
        soft.insert(String::from("maintener"), software_info[7]);
        soft.insert(String::from("mainurl"), software_info[8]);
        soft.insert(String::from("helpurl"), software_info[9]);
        // it''s more id than uuid
        soft.insert(String::from("guid"), software_info[10]);
        softwares.push(soft);
    }
    softwares
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
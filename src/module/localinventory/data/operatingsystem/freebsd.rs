#![cfg(target_os = "freebsd")]
use std::fs;
use chrono::prelude::{DateTime, Utc};
// use std::process::Command;
use crate::module::localinventory::data::operatingsystem::common::*;

pub fn run_inventory() -> serde_json::Value {
    let osname: String = get_os_name();
    let osversion: String = get_os_version();
    // split os_version in OS version + servicepack/patch level

    let cpuarch: String = get_cpu_arch();
    let hostname: String = get_hostname();
    let lastboottime: u64 = get_boottime();
    let timezone: String = get_timezone_name();
    let timezoneoffset: i32 = get_timezone_offset();
    let installdate: String = get_installation_date();

    // perhaps get the KELNEL ident: uname -i
    // kernel version: uname -k
    // 
    return fill_properties(osname, osversion, cpuarch, hostname, lastboottime, timezone, timezoneoffset, installdate)
}

fn get_installation_date() -> String {
    match fs::metadata("/var/log/bsdinstall_log") {
        Ok(metadata) => {
            let dt: DateTime<Utc> = metadata.modified().unwrap().clone().into();
            dt.format("%Y-%m-%d").to_string()
        },
        Err(e) => {
            String::from("")
        }
    }
}

fn fill_properties(osname: String, osversion: String, cpuarch: String, hostname: String,
                   lastboottime: u64, timezone: String, timezoneoffset: i32, installdate: String) -> serde_json::Value {
    serde_json::json!([
        {
            "key": "completename",
            "value": osname
        },
        {
            "key": "version",
            "value": osversion
        },
        {
            "key": "architecture",
            "value": cpuarch
        },
        {
            "key": "servicepack",
            "value": ""
        },
        {
            "key": "installationdate",
            "value": installdate
        },
        {
            "key": "hostname",
            "value": hostname
        },
        {
            "key": "domain",
            "value": ""
        },
        {
            "key": "lastboottime",
            "type": "integer",
            "unit": "s",
            "value": lastboottime
        },
        {
            "key": "timezonename",
            "value": timezone
        },
        {
            "key": "timezoneutcoffset",
            "value": timezoneoffset
        }
    ])
}

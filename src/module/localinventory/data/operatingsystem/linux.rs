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
    let installdate: String = "".to_string();

    return fill_properties(osname, osversion, cpuarch, hostname, lastboottime, timezone, timezoneoffset, installdate)
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

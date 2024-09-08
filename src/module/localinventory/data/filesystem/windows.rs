#![cfg(target_os = "windows")]
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
use regex::Regex;
use std::collections::HashMap;
use sysinfo::Disks;

pub fn run_inventory() -> Vec<serde_json::Value> {
    fill_properties()
}

fn fill_properties() -> Vec<serde_json::Value> {
    let mut filesystems_prop = Vec::new();

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        filesystems_prop.push(serde_json::json!([
            {
                "key": "type",
                "value": disk.file_system().to_str()
            },
            {
                "key": "name",
                "value": disk.name().to_str()
            },
            {
                "key": "partition",
                "value": disk.mount_point()
            }
        ]));
    }

    return filesystems_prop;
}

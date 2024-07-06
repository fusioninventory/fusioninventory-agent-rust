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

pub fn run_inventory() -> Vec<serde_json::Value> {
    log::info!("Get physicaldisks information");

    let mut disks = Vec::new();

    let disksproperties: Vec<serde_json::Value> = get_data();
    for properties in disksproperties {
        let mut mydisk = serde_json::json!({
            "type": "physicaldisk",
            "properties": properties,
            "children": [],
            "connectedto": []
        });

        let mut children = Vec::new();

        let partitions = module::localinventory::structure::partition::run_inventory(properties);
        for partition in partitions {
            children.push(partition);
        }
    
        mydisk["children"] = serde_json::Value::Array(children);

        disks.push(mydisk);
    }
    return disks;
}

#[cfg(target_os = "linux")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::physicaldisk::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::physicaldisk::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() -> Vec<serde_json::Value> {
    // TODO
    let data = Vec::new();
    return data;
}

#[cfg(target_os = "macos")]
fn get_data() -> Vec<serde_json::Value> {
    // TODO
    let data = Vec::new();
    return data;
}

// "windows"
// "macos"
// "ios"
// "linux"
// "android"
// "freebsd"
// "dragonfly"
// "openbsd"
// "netbsd"

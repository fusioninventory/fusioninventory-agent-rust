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

pub fn run_inventory(disk: serde_json::Value) -> Vec<serde_json::Value> {
    log::info!("Get partitions information");

    let mut partitions = Vec::new();

    let partitionsproperties: Vec<serde_json::Value> = get_data(disk);

    for properties in partitionsproperties {
        let partition = serde_json::json!({
            "type": "partition",
            "properties": properties,
            "children": [],
            "connectedto": []
        });
    }
    return partitions;
}

#[cfg(target_os = "linux")]
fn get_data(disk: serde_json::Value) -> Vec<serde_json::Value> {
    module::localinventory::data::partition::linux::run_inventory(disk)
}

#[cfg(target_os = "freebsd")]
fn get_data(disk: serde_json::Value) -> Vec<serde_json::Value> {
    module::localinventory::data::partition::freebsd::run_inventory(disk)
}

#[cfg(target_os = "windows")]
fn get_data(disk: serde_json::Value) -> Vec<serde_json::Value> {
    // module::localinventory::data::partition::windows::run_inventory(disk)
    let data = Vec::new();
    return data;
}

// "windows"
// "macos"
// "ios"
// "linux"
// "android"//
// "freebsd"
// "dragonfly"
// "openbsd"
// "netbsd"

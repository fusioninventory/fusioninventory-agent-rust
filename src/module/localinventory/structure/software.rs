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
    log::info!("Get softwares information");

    let mut softwares = Vec::new();

    let softwareproperties = get_data();

    for properties in softwareproperties {
        let software = serde_json::json!({
            "type": "software",
            "properties": properties,
            "children": [],
            "connectedto": []
        });

        softwares.push(software);
    }
    // List of packages tools
    // https://github.com/sigoden/upt
    return softwares;
}

#[cfg(target_os = "linux")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::software::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::software::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() -> Vec<serde_json::Value> {
    // TODO
    let data = Vec::new();
    return data;
}

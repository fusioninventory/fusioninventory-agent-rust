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

pub fn run_inventory() -> serde_json::Value {
    log::info!("Get operating system information");

    let properties = get_data();

    let mut ossys = serde_json::json!({
        "type": "operatingsystem",
        "properties": properties,
        "children": [],
        "connectedto": []
    });

    // (children)
    // 'FusionInventory::Inventory::Structure::Software',
    // 'FusionInventory::Inventory::Structure::RemoteManagement',
    
    let mut children = Vec::new();

    let softwares = module::localinventory::structure::software::run_inventory();
    for software in softwares {
        children.push(software);
    }
    ossys["children"] = serde_json::Value::Array(children);

    return ossys;
}

#[cfg(target_os = "linux")]
fn get_data() -> serde_json::Value {
    module::localinventory::data::operatingsystem::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> serde_json::Value {
    module::localinventory::data::operatingsystem::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() -> serde_json::Value {
    module::localinventory::data::operatingsystem::windows::run_inventory()
}

#[cfg(target_os = "macos")]
fn get_data() -> serde_json::Value {
    return serde_json::json!([]);
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




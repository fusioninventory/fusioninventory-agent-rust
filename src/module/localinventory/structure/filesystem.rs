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

pub fn run_inventory(vol_type: String, vol_name: String) -> Vec<serde_json::Value> {
    log::info!("Get filesystem information");

    let mut filesystems = Vec::new();
    // println!("FILESYSTEM: {:?}", vol_name);

    // return filesystems;
    if vol_name == "\"zroot/ROOT/default\"" || vol_name == "\"root\"" {
        let properties = serde_json::json!([
            {
                "key": "name",
                "value": "/"
            }
        ]);

        let mut children = Vec::new();
        let operatingsystem: serde_json::Value = module::localinventory::structure::operatingsystem::run_inventory();
        children.push(operatingsystem);
        // partition["children"] = serde_json::Value::Array(children);

        filesystems.push(serde_json::json!({
            "type": "filesystem",
            "properties": properties,
            "children": children,
            "connectedto": []
        }));
    } else if (vol_type == "".to_string()) && vol_name == "".to_string() {
        // Manage flesystems not on volume, but directly on partition

        let filesystemsproperties = get_data_novolume();

        for properties in filesystemsproperties {
            let mut children: Vec<serde_json::Value> = Vec::new();
            for prop in properties.as_array() {
                for prop2 in prop {
                    if prop2["key"] == "name" && prop2["value"] == "/" {
                        let operatingsystem: serde_json::Value = module::localinventory::structure::operatingsystem::run_inventory();
                        children.push(operatingsystem);
                    }
                    if prop2["key"] == "partition" && prop2["value"] == "C:\\" {
                        let operatingsystem: serde_json::Value = module::localinventory::structure::operatingsystem::run_inventory();
                        children.push(operatingsystem);
                    }
                }
            }
            filesystems.push(serde_json::json!({
                "type": "filesystem",
                "properties": properties,
                "children": children,
                "connectedto": []
            }));
        }
    }

    return filesystems;
}

#[cfg(target_os = "linux")]
fn get_data_novolume() -> Vec<serde_json::Value> {
    module::localinventory::data::filesystem::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data_novolume() -> Vec<serde_json::Value> {
    module::localinventory::data::filesystem::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data_novolume() -> Vec<serde_json::Value> {
    module::localinventory::data::filesystem::windows::run_inventory()
}

#[cfg(target_os = "macos")]
fn get_data_novolume() -> Vec<serde_json::Value> {
    // TODO
    let data = Vec::new();
    return data;
}


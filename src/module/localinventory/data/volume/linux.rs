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
use regex::Regex;
use std::collections::HashMap;

pub fn run_inventory() -> Vec<serde_json::Value> {

    let vgs = get_vgs();
    fill_properties(vgs)
}

fn get_vgs() -> Vec<HashMap<String, String>> {
    let cmd = "vgs";
    let args = ["--noheading", "--nosuffix", "--units", "M", "-o", "vg_name,pv_count,lv_count,vg_attr,vg_size,vg_free,vg_uuid,vg_extent_size"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("vgs command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(e) => empty,
    };

    // parsing the command output to extract information needed
    let mut volume_groups: Vec<HashMap<String, String>> = Vec::new();

    let re: Regex = Regex::new(r"^(?:\s+)(\w*)(?:\s+)(\d+)(?:\s+)(\d+)(?:\s+)([\w-]+)(?:\s+)([\d\.]+)(?:\s+)([\d\.]+)(?:\s+)([\w-]+)(?:\s+)([\d\.]+)").unwrap();
    for line in data.lines() {
        if let Some(mat) = re.captures(line) {
            println!("VGS: {:?}", mat);
            let mut volume_groups_attr: HashMap<String, String> = HashMap::new();

            volume_groups_attr.insert(String::from("name"), mat[1].trim().to_string());
            volume_groups_attr.insert(String::from("uuid"), mat[7].trim().to_string());
            volume_groups_attr.insert(String::from("totalsize"), mat[5].to_string());
            volume_groups_attr.insert(String::from("allocatedsize"), "".to_string());
            volume_groups_attr.insert(String::from("freesize"), mat[6].to_string());
            volume_groups_attr.insert(String::from("allocatedpercentage"), "".to_string());
            volume_groups_attr.insert(String::from("health"), "ONLINE".to_string());
            volume_groups.push(volume_groups_attr);
        }
    }
    return volume_groups;
}

fn get_lvs(volume_group: String) -> Vec<HashMap<String, String>> {

    let cmd = "lvs";
    let args = ["--noheading", "--nosuffix", "--units", "M", "-o", "lv_name,vg_uuid,lv_attr,lv_size,lv_uuid,seg_count", volume_group.as_str()];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("zfs list -r command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(e) => empty,
    };

    let mut logical_volumes: Vec<HashMap<String, String>> = Vec::new();
    let re: Regex = Regex::new(r"^(?:\s+)(\w*)(?:\s+)([\w-]+)(?:\s+)([\w-]*)(?:\s+)([\d\.]*)(?:\s+)([\w-]*)(?:\s+)(\d+)").unwrap();
    for line in data.lines() {
        if let Some(mat) = re.captures(line) {
            let mut attributes: HashMap<String, String> = HashMap::new();
            attributes.insert(String::from("name"), mat[1].trim().to_string());
            attributes.insert(String::from("uuid"), mat[5].trim().to_string());
            attributes.insert(String::from("type"), "lv".to_string());
            attributes.insert(String::from("totalsize"), mat[4].to_string());
            attributes.insert(String::from("freesize"), "".to_string());
            logical_volumes.push(attributes);
        }
    }
    return logical_volumes;
}


fn fill_properties(vgs: Vec<HashMap<String, String>>) -> Vec<serde_json::Value> {

    let mut vgs_prop = Vec::new();

    for vg in vgs.iter() {
        let lvs = get_lvs(vg.get("name").unwrap().clone());
        let lvs_props = fill_properties_lv(lvs);
        vgs_prop.push(serde_json::json!([
            {
                "key": "type",
                "value": "vg"
            },
            {
                "key": "name",
                "value": vg.get("name")
            },
            {
                "key": "partitions",
                "type": "list",
                "value": []
            },
            {
                "key": "totalsize",
                "type": "integer",
                "unit": "M",
                "value": vg.get("totalsize")
            },
            {
                "key": "allocatedsize",
                "type": "integer",
                "unit": "M",
                "value": vg.get("allocatedsize")
            },
            {
                "key": "freesize",
                "type": "integer",
                "unit": "M",
                "value": vg.get("freesize")
            },
            {
                "key": "allocatedpercentage",
                "type": "integer",
                "unit": "%",
                "value": vg.get("allocatedpercentage")
            },
            {
                "key": "health",
                "value": vg.get("health") // from list: ONLINE, DEGRADED, FAULTED, OFFLINE, REMOVED, UNAVAIL
            },
            {
                "key": "_children",
                "value": lvs_props
            }
        ]));
    }
    return vgs_prop;
}

fn fill_properties_lv(lvs: Vec<HashMap<String, String>>) -> Vec<serde_json::Value> {
    let mut lvs_prop = Vec::new();

    for lv in lvs.iter() {
        lvs_prop.push(serde_json::json!([
            {
                "key": "type",
                "value": "lv"
            },
            {
                "key": "name",
                "value": lv.get("name")
            },
            {
                "key": "partitions",
                "type": "list",
                "value": []
            },
            {
                "key": "totalsize",
                "type": "integer",
                "unit": "M",
                "value": lv.get("totalsize")

            },
            {
                "key": "allocatedsize",
                "type": "integer",
                "unit": "M",
                "value": ""
            },
            {
                "key": "freesize",
                "type": "integer",
                "unit": "M",
                "value": lv.get("freesize")
            },
            {
                "key": "allocatedpercentage",
                "type": "integer",
                "unit": "%",
                "value": ""
            },
            {
                "key": "health",
                "value": ""
            }
        ]));
    }
    return lvs_prop;
}


// volume: zpool
//   volume: dataset
//      filesystem

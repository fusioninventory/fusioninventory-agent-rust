#![cfg(target_os = "freebsd")]
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

use sysctl::Sysctl;

pub fn run_inventory(disk: serde_json::Value) -> Vec<serde_json::Value> {
    let xml = load_geom_data_xml();
    match xml {
        Some(x) => {
            return fill_properties(disk, x);
        }
        None => {
            println!("No data");
            return Vec::<serde_json::Value>::new();
        }
    }
}

fn load_geom_data_xml() -> Option<simple_xml::Node> {

    let ctl = match sysctl::Ctl::new("kern.geom.confxml") {
        Ok(f) => f,
        Err(e) => {
            log::error!("Read sysctl kern.geom.confxml have error: {:?}", e);
            return None;
        }
    };

    let val = ctl.value_string().unwrap();
    match simple_xml::from_string(val.as_str()) {
        Ok(f) => Some(f),
        Err(e) => {
            log::error!("Unable to read XML string of kern.geom.confxml: {:?}", e);
            None
        }
    }
}

fn fill_properties(disk: serde_json::Value, xml: simple_xml::Node) -> Vec<serde_json::Value> {

    let mut parts = Vec::new();
    let mut disk_id = String::from("test");
    let disks = disk.as_array();
    for disk in disks {
        for property in disk {
            if property["key"] == "id" {
                disk_id = property["value"].to_string();
                break;
            }
        }
    }
    log::debug!("THE ID {}", disk_id);

    for i in &xml["class"] {
        if i["name"][0].content == "PART" {
            for j in &i["geom"] {
                let provider = &j["consumer"][0]["provider"][0];
                if disk_id.trim_matches('"') == provider.attributes["ref"].trim_end_matches('"') {
                    for p in &j["provider"] {
                        let data = serde_json::json!([
                            {
                                "key": "id",
                                "value": p["config"][0]["rawuuid"][0].content,
                            },
                            {
                                "key": "creationdate",
                                "value": "",
                            },
                            {
                                "key": "description",
                                "value": "",
                            },
                            {
                                "key": "size",
                                "type": "integer",
                                "unit": "B",
                                "value": p["config"][0]["length"][0].content,
                            },
                            {
                                "key": "usedsize",
                                "value": "",
                            },
                            {
                                "key": "freesize",
                                "value": "",
                            },
                            {
                                "key": "filesystem",
                                "value": "",
                            },
                            {
                                "key": "label",
                                "value": p["config"][0]["label"][0].content,
                            },
                            {
                                "key": "mountpoint",
                                "value": "",
                            },
                            {
                                "key": "serial",
                                "value": "",
                            },
                            {
                                "key": "system",
                                "value": "",
                            },
                            {
                                "key": "ostype",
                                "value": p["config"][0]["type"][0].content,
                            },
                            // GET INFO in <name>ELI</name> class
                            {
                                "key": "encryption",
                                "value": "",
                            },
                            {
                                "key": "algorithm",
                                "value": "",
                            },
                            {
                                "key": "encryptedstatus",
                                "value": "",
                            },
                            {
                                "key": "encryptedtype",
                                "value": "",
                            }
                        ]);
                        parts.push(data);
                    }
                }
            }      
        }
    }
    return parts;
}

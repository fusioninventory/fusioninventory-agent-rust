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
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub struct FinalProcessorStruct {
    pub(crate) manufacturer: String,
    pub(crate) id: String,
    pub(crate) family: String,
    pub(crate) speed: i32,
    pub(crate) speed_unit: String,
    pub(crate) serialnumber: String,
    pub(crate) corecount: i32,
    pub(crate) threadcount: i32,
    pub(crate) flags: Vec<String>,
    pub(crate) characteristics: Vec<String>,
    pub(crate) l1cache: i32,
    pub(crate) l1cache_unit: String,
    pub(crate) l2cache: i32,
    pub(crate) l2cache_unit: String,
    pub(crate) l3cache: i32,
    pub(crate) l3cache_unit: String,
}

pub fn run_inventory() -> Vec<serde_json::Value> {
    log::info!("Get Chassis information");

    let mut cpus = Vec::new();

    let output: Vec<FinalProcessorStruct> = module::localinventory::data::cpu::dmidecode::run_inventory();
    let cpuproperties = fill_properties(output);

    for properties in cpuproperties {
        let cpu = serde_json::json!({
            "type": "cpu",
            "properties": properties,
            "children": [],
            "connectedto": []
        });

        cpus.push(cpu);
    }
    return cpus;
}

fn fill_properties(cpudata: Vec<FinalProcessorStruct>) -> Vec<serde_json::Value> {
    let mut cpus = Vec::new();

    for package in cpudata {
        let data = serde_json::json!([
            {
                "key": "manufacturer",
                "value": package.manufacturer
            },
            {
                "key": "id",
                "value": package.id
            },
            {
                "key": "family",
                "value": package.family
            },
            {
                "key": "speed",
                "type": "integer",
                "unit": package.speed_unit,
                "value": package.speed
            },
            {
                "key": "serialnumber",
                "value": package.serialnumber
            },
            {
                "key": "corecount",
                "type": "integer",
                "value": package.corecount
            },
            {
                "key": "threadcount",
                "type": "integer",
                "value": package.threadcount
            },
            {
                "key": "flags",
                "type": "list",
                "value": package.flags
            },
            {
                "key": "characteristics",
                "type": "list",
                "value": package.characteristics
            },
            {
                "key": "L1 cache",
                "type": "integer",
                "unit": package.l1cache_unit,
                "value": package.l1cache
            },
            {
                "key": "L2 cache",
                "type": "integer",
                "unit": package.l2cache_unit,
                "value": package.l2cache
            },
            {
                "key": "L2 cache",
                "type": "integer",
                "unit": package.l3cache_unit,
                "value": package.l3cache
            }
        ]);
        cpus.push(data);
    }
    return cpus;

}

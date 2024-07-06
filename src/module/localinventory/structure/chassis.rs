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

// struct manufacturer {
//     "key": "manufacturer",
//     "value": String
// };

// struct chassis {
//     "key": "chassis",
//     "value": String
// };

// struct serialnumber {
//     "key": "serialnumber",
//     "value": String
// };

// struct model {
//     "key": "model",
//     "value": String
// };

// struct uuid {
//     "key": "uuid",
//     "value": String
// };

// struct Data {
//     "type": String,
//     properties: [
//       manufacturer,
//       chassis,
//       serialnumber,
//       model,
//       uuid
//     ],
//     children => [],
//     connectedto => []
// };

pub fn run_inventory() -> serde_json::Value {
    log::info!("Get Chassis information");

    let properties = module::localinventory::data::chassis::dmidecode::run_inventory();

    let mut chassis = serde_json::json!({
        "type": "chassis",
        "properties": properties,
        "children": [],
        "connectedto": []
    });

    let mut children = Vec::new();

    // 'FusionInventory::Inventory::Structure::MemorySlot',

    // Get CPUs
    let cpus = module::localinventory::structure::cpu::run_inventory();
    for cpu in cpus {
        children.push(cpu);
    }

    // Get disks
    let disks = module::localinventory::structure::physicaldisk::run_inventory();
    for disk in disks {
        children.push(disk);
    }

    // Get volumes
    let volumes = module::localinventory::structure::volume::run_inventory();
    for volume in volumes {
        children.push(volume);
    }

    // get filesystem directly on partition, not on volumes
    let filesystems = module::localinventory::structure::filesystem::run_inventory("".to_string(), "".to_string());
    for fs in filesystems {
        children.push(fs);
    }

    chassis["children"] = serde_json::Value::Array(children);

    log::debug!("Local inventory: {}", serde_json::to_string_pretty(&chassis).unwrap());
    return chassis;

    // TODO when finish, we can delete all temp files created on disk
}

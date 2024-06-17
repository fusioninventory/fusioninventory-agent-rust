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
    log::info!("Get Chassis information");

    let properties = module::localinventory::data::cpu::dmidecode::run_inventory();

    let cpu = serde_json::json!({
        "type": "CPU",
        "properties": properties,
        "children": [],
        "connectedto": []
    });

    return cpu;
}

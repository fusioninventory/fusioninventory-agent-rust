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

pub struct FinalPackageStruct {
    pub(crate) name: String,
    pub(crate) originalname: String,
    pub(crate) version: String,
    pub(crate) revision: String,
    pub(crate) originalversion: String,
    pub(crate) publisher: String,
    pub(crate) maintainer: String,
    pub(crate) repository: String,
    pub(crate) pkgtype: String,
    pub(crate) architecture: String,
    pub(crate) category: String,
    pub(crate) installationdate: String,
    pub(crate) uninstallcommand: String,
    pub(crate) id: String,
    pub(crate) comment: String,
    pub(crate) mainurl: String,
    pub(crate) helpurl: String,
}


pub fn run_inventory() -> Vec<serde_json::Value> {
    log::info!("Get softwares information");

    let mut softwares = Vec::new();

    let data: Vec<FinalPackageStruct> = get_data();
    let softwareproperties = fill_properties(data);

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

fn fill_properties(packages: Vec<FinalPackageStruct>) -> Vec<serde_json::Value> {
    let mut softwares = Vec::new();

    for package in packages {
        let data = serde_json::json!([
            {
                "key": "name",
                "value": package.name,
            },
            {
                "key": "originalname",
                "value": package.originalname,
            },
            {
                "key": "version",
                "value": package.version,
            },
            {
                "key": "revision",
                "value": package.revision,
            },
            {
                "key": "originalversion",
                "value": package.originalversion,
            },
            {
                "key": "publisher",
                "value": package.publisher,
            },
            {
                "key": "maintainer",
                "value": package.maintainer,
            },
            {
                "key": "repository",
                "value": package.repository,
            },
            {
                "key": "type",
                "value": package.pkgtype, // pkg, PRM, ...
            },
            {
                "key": "architecture",
                "value": package.architecture,
            },
            {
                "key": "category",
                "value": package.category
            },
            {
                "key": "installationdate",
                "value": package.installationdate,
            },
            {
                "key": "uninstallcommand",
                "value": package.uninstallcommand,
            },
            {
                "key": "id",
                "value": package.id,
            },
            {
                "key": "comment",
                "value": package.comment,
            },
            {
                "key": "mainurl",
                "value": package.mainurl,
            },
            {
                "key": "helpurl",
                "value": package.helpurl,
            }
        ]);
        softwares.push(data);
    }
    return softwares;
}

#[cfg(target_os = "linux")]
fn get_data() -> Vec<FinalPackageStruct> {
    let mut softwares = Vec::new();

    // https://github.com/sigoden/upt

    let mut data_rpm = module::localinventory::data::software::linux::rpm::run_inventory();
    softwares.append(&mut data_rpm);

    let mut data_dpkg = module::localinventory::data::software::linux::dpkg::run_inventory();
    softwares.append(&mut data_dpkg);

    let mut data_pacman = module::localinventory::data::software::linux::pacman::run_inventory();
    softwares.append(&mut data_pacman);

    let mut data_nix = module::localinventory::data::software::linux::nix::run_inventory();
    softwares.append(&mut data_nix);

    let mut data_snap = module::localinventory::data::software::linux::snap::run_inventory();
    softwares.append(&mut data_snap);

    return softwares;
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<FinalPackageStruct> {
    module::localinventory::data::software::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() -> Vec<FinalPackageStruct> {
    // TODO
    module::localinventory::data::software::windows::registry::run_inventory();

    let data = Vec::new();
    return data;
}

#[cfg(target_os = "macos")]
fn get_data() -> Vec<FinalPackageStruct> {
    // TODO
    let data = Vec::new();
    return data;
}

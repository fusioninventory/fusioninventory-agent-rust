#![cfg(target_os = "windows")]
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
use module::localinventory::structure::software::FinalPackageStruct;
use winreg::enums::*;
use winreg::RegKey;

pub fn run_inventory() -> Vec<FinalPackageStruct> {
    get_registry_softwares()
}

fn get_registry_softwares() -> Vec<FinalPackageStruct> {
    let mut softwares: Vec<FinalPackageStruct> = Vec::new();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_path = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall");
    
    // let info = cur_path.query_info();
    if let Ok(x) = cur_path {
        for nm in x.enum_keys().map(|x| x.unwrap()) {
            let subkey = x.open_subkey(&nm).unwrap();

            let name: String = subkey.get_value("DisplayName").unwrap_or_default();
            if name == "" {
                continue;
            }

            let my_software: FinalPackageStruct = FinalPackageStruct {
                name: name.clone(),
                originalname: name,
                version: subkey.get_value("DisplayVersion").unwrap_or_default(),
                revision: "".to_string(),
                originalversion: subkey.get_value("DisplayVersion").unwrap_or_default(),
                publisher: subkey.get_value("Publisher").unwrap_or_default(),
                maintainer: subkey.get_value("Publisher").unwrap_or_default(),
                repository: "".to_string(),
                pkgtype: "".to_string(),
                architecture: "".to_string(),
                category: "".to_string(),
                installationdate: subkey.get_value("InstallDate").unwrap_or_default(),
                uninstallcommand: "".to_string(),
                id: "".to_string(),
                comment: subkey.get_value("Comments").unwrap_or_default(),
                mainurl: subkey.get_value("URLInfoAbout").unwrap_or_default(),
                helpurl: subkey.get_value("HelpLink").unwrap_or_default()
            };
            softwares.push(my_software);
        }
    };
    return softwares;
}

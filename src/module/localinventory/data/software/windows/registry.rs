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
    // let output_snap = get_snap_list();
    let data: Vec<FinalPackageStruct> = Vec::new();
    return data;
}

fn get_registry_softwares() {

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_path = hklm.open_subkey("SOFTWARE/Microsoft/Windows/CurrentVersion/Uninstall");

    // path    => 'HKEY_LOCAL_MACHINE/SOFTWARE/Microsoft/Windows/CurrentVersion/Uninstall',
    // wmiopts => { # Only used for remote WMI optimization
    //   values  => [ qw/
    //     DisplayName Comments HelpLink ReleaseType DisplayVersion
    //     Publisher URLInfoAbout UninstallString InstallDate MinorVersion
    //     MajorVersion NoRemove SystemComponent
    //     / ]
    // },

}

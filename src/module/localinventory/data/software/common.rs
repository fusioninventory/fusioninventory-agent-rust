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

use regex::Regex;

pub fn clean_name(name: String) -> String {
    // TODO try use github.com/rust-ml/linfa
        // intel MKL is only the one run on windows, linux and macos





    let re: Regex = Regex::new(r"^([a-zA-Z\-]+)([\-\d\.]+)$").unwrap();
    let redash: Regex = Regex::new(r"(.*)\-$").unwrap();
    if let Some(mat) = re.captures(name.as_str()) {
        if let Some(matdash) = redash.captures(mat[1].trim()) {
            return matdash[1].trim().to_string();
        }
        return mat[1].trim().to_string();
    }
    return name;
}

pub fn clean_version(version: String) -> String {

    let re: Regex = Regex::new(r"^(\S*)([\-\d\.]+)$").unwrap();
    if let Some(mat) = re.captures(version.as_str()) {
        return mat[1].trim().to_string();
    }
    return version;
}

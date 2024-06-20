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

use crate::CONFIG;
use crate::module;
use std::fs::File;
use std::io::prelude::*;
use reqwest::header::USER_AGENT;

const USER_AGENT_VALUE: &str = "FusionInventort-agent v3.0.0";

pub fn main() {
    // get servers in config and loop
    for server in CONFIG.localinventory.servers.clone().into_iter() {
        if is_http(server.clone()) {
            run_http(server);
        } else {
            run_path(server);
        }
    }
}

fn is_http(server: String) -> bool {
    if server.starts_with("http") {
        return true;
    }
    return false;
}

fn run_http(server: String) {
    let inventory = module::localinventory::structure::chassis::run_inventory();
    //TODO
    // do request in GET of the URL
    // manage modifications of config
    // do inventory
    // send data on same URL but in POST

    let client = reqwest::blocking::Client::new();

    match client.post(server)
        .header(USER_AGENT, USER_AGENT_VALUE)
        .json(&inventory)
        .send() {
            Ok(r) => {
                println!("Response: {:?} {}", r.version(), r.status());
                println!("Headers: {:#?}\n", r.headers());
            },
            Err(c) => println!("ERROR: {}", c),
        };

    // let client = reqwest::Client::new();
    // let res = client.post(server)
    //     .header(USER_AGENT, USER_AGENT_VALUE)
    //     .json(&inventory)
    //     .send()
    //     .await;
    // println!("Result: {:?}", res);
    //     // .await {
    //     // each response is wrapped in a `Result` type
    //     // we'll unwrap here for simplicity

    //     .text() {
    //     // .await;
    //         Ok(r) => println!("hhtp return: {:?}", r),
    //         Err(e) => println!("http error: {:?}", e),
    // };
}

fn run_path(server: String) {
    println!("{}", server);
    let inventory = module::localinventory::structure::chassis::run_inventory();
    let mut file = match File::create(server.clone() + "/testinventory.json") {
        Ok(r) => r,
        Err(e) => {
            println!("Error writing file {}", e);
            return;
        }
    };
    let _ = file.write_all(serde_json::to_string_pretty(&inventory).unwrap().as_bytes());

    let mut file = match File::create(server + "/testinventory_condensed.json") {
        Ok(r) => r,
        Err(e) => {
            println!("Error writing file {}", e);
            return;
        }
    };
    let _ = file.write_all(serde_json::to_string(&inventory).unwrap().as_bytes());

}


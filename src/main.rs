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

#[macro_use] extern crate rocket;

use local_inventory_sender::send_inventory;
use std::env;
use clap::Parser;
use lazy_static::lazy_static;
use crate::common::config::Data;

mod local_inventory_sender;
mod module;
mod common;
use std::sync::mpsc;
use std::sync::Mutex;

// Manage configuration
lazy_static! {
    pub static ref CONFIG: Data = common::config::main();
    static ref CHANNEL: (mpsc::SyncSender<String>, Mutex<mpsc::Receiver<String>>) = common::webserver::init_channel();
    static ref LOCALINVENTORYNEXT: (Mutex<single_value_channel::Receiver<u64>>, single_value_channel::Updater<u64>) = common::webserver::init_channel_u64();
    static ref NETWORKDISCOVERYNEXT: (Mutex<single_value_channel::Receiver<u64>>, single_value_channel::Updater<u64>) = common::webserver::init_channel_u64();
    static ref NETWORKINVENTORYNEXT: (Mutex<single_value_channel::Receiver<u64>>, single_value_channel::Updater<u64>) = common::webserver::init_channel_u64();
    static ref DEPLOYNEXT: (Mutex<single_value_channel::Receiver<u64>>, single_value_channel::Updater<u64>) = common::webserver::init_channel_u64();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,

    /// Run in daemon mode
    #[arg(short, long)]
    daemon: bool,

    /// Run in debug mode
    #[arg(long)]
    debug: bool,

}

struct NextTimes {
    localinventory: u64,
    networkdiscovery: u64,
    networkinventory: u64,
    deploy: u64,
}

fn main() {
    let args = Args::parse();
    if args.debug || CONFIG.logging.logger_level == "debug" {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    println!("args: {:?}", args);
    // return;

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    ctrlc::set_handler(move || {
        println!("received Ctrl+C, exit FusionInventory-agent");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // TODO load from state file
    module::common::run_modules_in_thread();
   
    send_inventory();
}

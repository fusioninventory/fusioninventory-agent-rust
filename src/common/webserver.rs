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

use rocket::Rocket;
use rocket::Build;
use rocket::response::{content, Redirect};
use rocket::fs::{FileServer, relative};
use crate::CONFIG;
use crate::CHANNEL;
use crate::{LOCALINVENTORYNEXT, NETWORKDISCOVERYNEXT, NETWORKINVENTORYNEXT, DEPLOYNEXT};
use std::sync::{mpsc, Mutex};
use single_value_channel::channel_starting_with;

use chrono::prelude::DateTime;
use chrono::Local;
use std::time::{UNIX_EPOCH, Duration};

pub fn init_channel() -> (mpsc::SyncSender<String>, Mutex<mpsc::Receiver<String>>) {
    let channel: (mpsc::SyncSender<String>, mpsc::Receiver<String>) = mpsc::sync_channel(0);
    (channel.0, Mutex::new(channel.1))
}

pub fn init_channel_u64() -> (Mutex<single_value_channel::Receiver<u64>>, single_value_channel::Updater<u64>) {
    let channel = channel_starting_with(0);
    (Mutex::new(channel.0), channel.1)
}

#[get("/")]
fn default() -> Redirect {
    // redirect to status page
    Redirect::to("/status")
}

#[get("/status")]
fn status() -> content::RawHtml<String> {
    let mut localinventory = "disabled";
    let mut networkdiscovery = "disabled";
    let mut networkinventory = "disabled";
    let mut deploy = "disabled";

    let receiver_nc: std::sync::MutexGuard<'_, single_value_channel::Receiver<u64>> = LOCALINVENTORYNEXT.0.lock().unwrap();
    let date_localinventory: String = get_next_date(receiver_nc, CONFIG.localinventory.enabled);

    let receiver_nc: std::sync::MutexGuard<'_, single_value_channel::Receiver<u64>> = NETWORKDISCOVERYNEXT.0.lock().unwrap();
    let date_networkdiscovery: String = get_next_date(receiver_nc, CONFIG.networkdiscovery.enabled);

    let receiver_nc: std::sync::MutexGuard<'_, single_value_channel::Receiver<u64>> = NETWORKINVENTORYNEXT.0.lock().unwrap();
    let date_networkinventory: String = get_next_date(receiver_nc, CONFIG.networkinventory.enabled);

    let receiver_nc: std::sync::MutexGuard<'_, single_value_channel::Receiver<u64>> = DEPLOYNEXT.0.lock().unwrap();
    let date_deploy: String = get_next_date(receiver_nc, CONFIG.deploy.enabled);

    if CONFIG.localinventory.enabled {
        localinventory = "enabled";
    }
    if CONFIG.networkdiscovery.enabled {
        networkdiscovery = "enabled";
    }
    if CONFIG.networkinventory.enabled {
        networkinventory = "enabled";
    }
    if CONFIG.deploy.enabled {
        deploy = "enabled";
    }

    content::RawHtml(format!("
<html>
    <head>
        <title>FusionInventory: status page</title>
        <link rel=\"icon\" href=\"/img/favicon.ico\" />
        <link rel=\"stylesheet\" href=\"/css/main.css\"> 
    </head>
    <body>
        <div class=\"logo\">
        <img src=\"/img/logo_fusioninventory.png\"><br>
        </div>
        <table>
            <tr>
                <th>Module</th>
                <th>status</th>
                <th>link to force running now</th>
                <th>next execution planned</th>
            </tr>
            <tr>
                <td>Localinventory</td>
                <td>{localinventory}</td>
                <td><a href=\"/now/localinventory\">run now</a></td>
                <td>{date_localinventory}</td>
            </tr>
            <tr>
                <td>Networkdiscovery</td>
                <td>{networkdiscovery}</td>
                <td><a href=\"/now/networkdiscovery\">run now</a></td>
                <td>{date_networkdiscovery}</td>
            </tr>
            <tr>
                <td>Networkinventory</td>
                <td>{networkinventory}</td>
                <td><a href=\"/now/networkinventory\">run now</a></td>
                <td>{date_networkinventory}</td>
            </tr>
            <tr>
                <td>Deploy</td>
                <td>{deploy}</td>
                <td><a href=\"/now/deploy\">run now</a></td>
                <td>{date_deploy}</td>
            </tr>
        </table>
    </body>
</html>
"))
}

#[get("/localinventory")]
fn run_localinventory() -> content::RawHtml<String> {
    // check if not running
    let receiver = LOCALINVENTORYNEXT.0.lock().unwrap();
    let next_date: String = get_next_date(receiver, CONFIG.localinventory.enabled);
    set_run_page(next_date, "localinventory".to_string())
}

#[get("/networkdiscovery")]
fn run_networkdiscovery() -> content::RawHtml<String> {
    // check if not running
    let receiver = NETWORKDISCOVERYNEXT.0.lock().unwrap();
    let next_date: String = get_next_date(receiver, CONFIG.networkdiscovery.enabled);
    set_run_page(next_date, "networkdiscovery".to_string())
}

#[get("/networkinventory")]
fn run_networkinventory() -> content::RawHtml<String> {
    // check if not running
    let receiver = NETWORKINVENTORYNEXT.0.lock().unwrap();
    let next_date: String = get_next_date(receiver, CONFIG.networkinventory.enabled);
    set_run_page(next_date, "networkinventory".to_string())
}

#[get("/deploy")]
fn run_deploy() -> content::RawHtml<String> {
    // check if not running
    let receiver = DEPLOYNEXT.0.lock().unwrap();
    let next_date: String = get_next_date(receiver, CONFIG.deploy.enabled);
    set_run_page(next_date, "deploy".to_string())
}

#[rocket::launch]
pub fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment()
        .merge(("port", CONFIG.webinterface.port));

    rocket::custom(figment)
        .mount("/", routes![status, default])
        .mount("/now", routes![run_localinventory, run_networkdiscovery, run_networkinventory, run_deploy])
        .mount("/css", FileServer::from("static/webserver/css"))
        .mount("/img", FileServer::from("static/webserver/img"))
        // create /api/status to have in json format
}

fn get_next_date(mut receiver: std::sync::MutexGuard<'_, single_value_channel::Receiver<u64>>, enabled: bool) -> String {
    let localinventory_next = receiver.latest();
    let mut timestamp_str: String = "running".to_string();
    if localinventory_next > &0 {
        let d = UNIX_EPOCH + Duration::from_secs(localinventory_next.clone());
        // Create DateTime from SystemTime
        let datetime = DateTime::<Local>::from(d);
        // Formats the combined date and time with the specified format string.
        timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!{"Date: {}",timestamp_str};
    }
    if !enabled {
        timestamp_str = "disabled".to_string();
    }
    return timestamp_str;
}

fn set_run_page(next_date: String, modulename: String) -> content::RawHtml<String> {
    let content: content::RawHtml<String> = match next_date.as_str() {
        "disabled" => {
            content::RawHtml(format!("<h2>Sorry, this module is disabled by configuration</h2>"))
        },
        "running" => {
            content::RawHtml(format!("<h2>{modulename} is actualy running, I do nothing</h2>"))
        },
        _ => {
            let content = content::RawHtml(format!("<h1>Run {modulename}, Baby!</h1>"));
            let val = String::from(modulename);
            CHANNEL.0.send(val).unwrap();
            return content;
        },
    };
    return content;
}

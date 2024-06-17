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

use serde_json::json;
use reqwest::header::USER_AGENT;
use crate::CONFIG;

const USER_AGENT_VALUE: &str = "FusionInventort-agent v3.0.0";
const SERVER_URL: &str = "http://127.0.0.1/fusionsuite/backend/v1/fusioninventory/localinventory";

#[tokio::main]
pub async fn send_inventory() {
println!("TEST CONFIG: {:?}", CONFIG.localinventory.servers);
println!("TEST CONFIG: {:?}", CONFIG.localinventory.enabled);

    // let mut map = HashMap::new();
    // map.insert("lang", "rust");
    // map.insert("body", "json");
    let data = &json!([
        {
            "type": "chassis",
            "properties": [
                {
                    "key": "manufacturer",
                    "value": "Dell"
                },
                {
                    "key": "chassis",
                    "value": "Dell"
                },
                {
                    "key": "serialnumber",
                    "value": "XXHTY"
                },
                {
                    "key": "model",
                    "value": ""
                },
                {
                    "key": "uuid",
                    "value": ""
                }
            ],
            "children": [],
            "connectedto": []
        }
    ]);

    let client = reqwest::Client::new();
    let res = client.post(SERVER_URL)
        .header(USER_AGENT, USER_AGENT_VALUE)
        .json(data)
        .send()
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap()
        .text()
        .await;

    println!("{:?}", res);

    // match res.status() {
    //     reqwest::StatusCode::OK => {
    //         // on success, parse our JSON to an APIResponse
    //         match res.json::().await {
    //             Ok(parsed) => println!("Success! {:?}", parsed),
    //             Err(_) => println!("Hm, the response didn't match the shape we expected."),
    //         };
    //     }
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Need to grab a new token");
    //     }
    //     other => {
    //         panic!("Uh oh! Something unexpected happened: {:?}", other);
    //     }
    // };


    // match res {
    //     Ok(..) => {
    //         // let response_text = res.text().await?;
    //         info!("Processing order response");
    //         // let mut futures = vec![];
    //         // for result in order.result {
    //         //     let future = task::spawn(process_order(result));
    //         //     futures.push(future);
    //         // }

    //         // join_all(futures).await;
    //     }
    //     Err(e) => {
    //         error!("Orders API response cannot be parsed! {}", e)
    //     }
    // };
}

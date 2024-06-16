use crate::module;

pub fn run_inventory() -> Vec<serde_json::Value> {
    log::info!("Get volumes information");

    let volumesproperties: Vec<serde_json::Value> = get_data();

    let data = loop_properties(volumesproperties);

    // for properties in volumesproperties {
    //     // new_properties = properties.clone();
    //     // TODO delete children
    //     for prop in properties.as_array() {
    //         for prop2 in prop {
    //             if prop2["key"] == "_children" {
    //                 println!("CHILDREN HERE !");

    //             } else {
    //                 println!("TEST my prop  ****: {:?}", prop2);
    //             }
    //         }
    //     }
    //     let mut myvolume = serde_json::json!({
    //         "type": "volume",
    //         "properties": properties,
    //         "children": [],
    //         "connectedto": []
    //     });

    //     let mut children = Vec::new();

    //     // let partitions = module::localinventory::structure::partition::run_inventory(properties);
    //     // for partition in partitions {
    //     //     children.push(partition);
    //     // }
    
    //     myvolume["children"] = serde_json::Value::Array(children);

    //     volumes.push(myvolume);
    // }
    // return volumes;
    return data;

}

fn loop_properties(volumesproperties: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    let mut volumes = Vec::new();

    for volume_properties in volumesproperties {
        let mut fill_properties: Vec<serde_json::Value> = Vec::new();
        let mut children: Vec<serde_json::Value> = Vec::new();
        let mut vol_type = "".to_string();
        let mut vol_name = "".to_string();
        for prop in volume_properties.as_array() {
            for prop2 in prop {
                if prop2["key"] == "_children" {
                    children = loop_properties(prop2["value"].as_array().unwrap().clone());
                } else {
                    fill_properties.push(prop2.clone());
                    if prop2["key"] == "type" {
                        vol_type = prop2["value"].to_string();
                    }
                    if prop2["key"] == "name" {
                        vol_name = prop2["value"].to_string();
                    }
                }
            }
        }
        if children.iter().count() == 0 {
            // TODO get filesystems
            let filesystem = module::localinventory::structure::filesystem::run_inventory(vol_type, vol_name);
            if filesystem.iter().count() > 0 {
                children.push(filesystem.into_iter().next().unwrap());
            }
        }

        volumes.push(serde_json::json!({
            "type": "volume",
            "properties": fill_properties,
            "children": children,
            "connectedto": []
        }));
    }
    return volumes;
}

#[cfg(target_os = "linux")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::volume::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::volume::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() -> Vec<serde_json::Value> {
    // module::localinventory::data::partition::windows::run_inventory(disk)
    let data = Vec::new();
    return data;
}

// "windows"
// "macos"
// "ios"
// "linux"
// "android"//
// "freebsd"
// "dragonfly"
// "openbsd"
// "netbsd"

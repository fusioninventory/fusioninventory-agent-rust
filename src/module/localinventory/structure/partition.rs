use crate::module;

pub fn run_inventory(disk: serde_json::Value) -> Vec<serde_json::Value> {
    log::info!("Get partitions information");

    let mut partitions = Vec::new();

    let partitionsproperties: Vec<serde_json::Value> = get_data(disk);

    for properties in partitionsproperties {
        let mut partition = serde_json::json!({
            "type": "partition",
            "properties": properties,
            "children": [],
            "connectedto": []
        });

        let mut children = Vec::new();

        // // (children)
        // // TODO check if OS installed on this partition
        let operatingsystem: serde_json::Value = module::localinventory::structure::operatingsystem::run_inventory();
        children.push(operatingsystem);

        partition["children"] = serde_json::Value::Array(children);
        partitions.push(partition);
    }
    return partitions;
}

#[cfg(target_os = "linux")]
fn get_data(disk: serde_json::Value) -> Vec<serde_json::Value> {
    module::localinventory::data::partition::linux::run_inventory(disk)
}

#[cfg(target_os = "freebsd")]
fn get_data(disk: serde_json::Value) -> Vec<serde_json::Value> {
    module::localinventory::data::partition::freebsd::run_inventory(disk)
}

#[cfg(target_os = "windows")]
fn get_data(disk: serde_json::Value) -> Vec<serde_json::Value> {
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

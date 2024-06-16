use crate::module;

pub fn run_inventory() -> Vec<serde_json::Value> {
    log::info!("Get physicaldisks information");

    let mut disks = Vec::new();

    let disksproperties: Vec<serde_json::Value> = get_data();
    for properties in disksproperties {
        let mut mydisk = serde_json::json!({
            "type": "physicaldisk",
            "properties": properties,
            "children": [],
            "connectedto": []
        });

        let mut children = Vec::new();

        let partitions = module::localinventory::structure::partition::run_inventory(properties);
        for partition in partitions {
            children.push(partition);
        }
    
        mydisk["children"] = serde_json::Value::Array(children);

        disks.push(mydisk);
    }
    return disks;
}

#[cfg(target_os = "linux")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::physicaldisk::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::physicaldisk::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() {
    
}

// "windows"
// "macos"
// "ios"
// "linux"
// "android"
// "freebsd"
// "dragonfly"
// "openbsd"
// "netbsd"

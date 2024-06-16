use crate::module;

pub fn run_inventory() -> serde_json::Value {
    log::info!("Get operating system information");

    let properties = get_data();

    let mut ossys = serde_json::json!({
        "type": "operatingsystem",
        "properties": properties,
        "children": [],
        "connectedto": []
    });

    // (children)
    // 'FusionInventory::Inventory::Structure::Software',
    // 'FusionInventory::Inventory::Structure::RemoteManagement',
    
    let mut children = Vec::new();

    let softwares = module::localinventory::structure::software::run_inventory();
    for software in softwares {
        children.push(software);
    }
    ossys["children"] = serde_json::Value::Array(children);

    return ossys;
}

#[cfg(target_os = "linux")]
fn get_data() -> serde_json::Value {
    module::localinventory::data::operatingsystem::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> serde_json::Value {
    module::localinventory::data::operatingsystem::freebsd::run_inventory()
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




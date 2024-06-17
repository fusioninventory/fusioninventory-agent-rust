use crate::module;

pub fn run_inventory() -> Vec<serde_json::Value> {
    log::info!("Get softwares information");

    let mut softwares = Vec::new();

    let softwareproperties = get_data();

    for properties in softwareproperties {
        let software = serde_json::json!({
            "type": "software",
            "properties": properties,
            "children": [],
            "connectedto": []
        });

        softwares.push(software);
    }
    // List of packages tools
    // https://github.com/sigoden/upt
    return softwares;
}

#[cfg(target_os = "linux")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::software::linux::run_inventory()
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::software::freebsd::run_inventory()
}

#[cfg(target_os = "windows")]
fn get_data() -> Vec<serde_json::Value> {
    // TODO
    let data = Vec::new();
    return data;
}

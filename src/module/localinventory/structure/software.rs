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
fn get_data() -> Vec<serde_json::Value>{
    module::localinventory::data::software::linux::run_inventory();
    let softwares = Vec::new();
    return softwares;
}

#[cfg(target_os = "freebsd")]
fn get_data() -> Vec<serde_json::Value> {
    module::localinventory::data::software::freebsd::run_inventory()
}

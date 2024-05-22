use crate::module;

pub fn run_inventory() -> serde_json::Value {
    log::info!("Get Chassis information");

    let properties = module::localinventory::data::cpu::dmidecode::run_inventory();

    let cpu = serde_json::json!({
        "type": "CPU",
        "properties": properties,
        "children": [],
        "connectedto": []
    });

    return cpu;
}

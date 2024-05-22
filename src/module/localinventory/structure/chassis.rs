use crate::module;

// struct manufacturer {
//     "key": "manufacturer",
//     "value": String
// };

// struct chassis {
//     "key": "chassis",
//     "value": String
// };

// struct serialnumber {
//     "key": "serialnumber",
//     "value": String
// };

// struct model {
//     "key": "model",
//     "value": String
// };

// struct uuid {
//     "key": "uuid",
//     "value": String
// };

// struct Data {
//     "type": String,
//     properties: [
//       manufacturer,
//       chassis,
//       serialnumber,
//       model,
//       uuid
//     ],
//     children => [],
//     connectedto => []
// };

pub fn run_inventory() {
    log::info!("Get Chassis information");

    let properties = module::localinventory::data::chassis::dmidecode::run_inventory();

    let mut chassis = serde_json::json!({
        "type": "chassis",
        "properties": properties,
        "children": [],
        "connectedto": []
    });

    let mut children = Vec::new();

    // 'FusionInventory::Inventory::Structure::MemorySlot',

    // Get CPUs
    let cpus = module::localinventory::structure::cpu::run_inventory();
    children.push(cpus);

    // Get disks
    let disks = module::localinventory::structure::physicaldisk::run_inventory();
    for disk in disks {
        children.push(disk);
    }

    chassis["children"] = serde_json::Value::Array(children);

    log::debug!("Local inventory: {}", serde_json::to_string_pretty(&chassis).unwrap());

    // TODO when finish, we can delete all temp files created on disk
}

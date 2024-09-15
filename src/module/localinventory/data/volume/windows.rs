#![cfg(target_os = "windows")]

use sysinfo::Disks;

pub fn run_inventory() -> Vec<serde_json::Value> {
  fill_properties()
}

fn fill_properties() -> Vec<serde_json::Value> {
  let mut vols = Vec::new();


  let disks = Disks::new_with_refreshed_list();
  for disk in &disks {
    vols.push(serde_json::json!([
      {
        "key": "type",
        "value": ""
      },
      {
        "key": "name",
        "value": disk.name().to_str()
      },
      {
        "key": "partitions",
        "type": "list",
        "value": []
      },
      {
        "key": "totalsize",
        "type": "integer",
        "unit": "B",
        "value": disk.total_space()
      },
      {
        "key": "allocatedsize",
        "type": "integer",
        "unit": "B",
        "value": (disk.total_space() - disk.available_space())
      },
      {
        "key": "freesize",
        "type": "integer",
        "unit": "B",
        "value": disk.available_space()
      },
      {
        "key": "allocatedpercentage",
        "type": "integer",
        "unit": "%",
        "value": 0
      },
      {
        "key": "health",
        "value": "ONLINE"
      }
    ]));
  }
  return vols;
}

#![cfg(target_os = "freebsd")]
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

// Copyright (C) 2024 FusionSuite Team
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// Affero General Public License as published by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details. You should have received a copy of the GNU Affero
// General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.



use std::process::{Command, Stdio};
use regex::Regex;
use std::collections::HashMap;

pub fn run_inventory() -> Vec<serde_json::Value> {

    // GET ZFS
// zpool status -P
//     pool: zroot
//    state: ONLINE
//   config:
  
//           NAME               STATE     READ WRITE CKSUM
//           zroot              ONLINE       0     0     0
//             /dev/nda0p4.eli  ONLINE       0     0     0
  
// # zpool list -L -P -v
// NAME                SIZE  ALLOC   FREE  CKPOINT  EXPANDSZ   FRAG    CAP  DEDUP    HEALTH  ALTROOT
// zroot               468G   110G   358G        -         -     9%    23%  1.00x    ONLINE  -
//   /dev/nda0p4.eli   469G   110G   358G        -         -     9%  23.5%      -    ONLINE



// zfs list
// NAME                                        USED  AVAIL  REFER  MOUNTPOINT
// zroot                                       110G   344G    96K  /zroot
// zroot/ROOT                                 19.6G   344G    96K  none
// zroot/ROOT/14.0-RELEASE_2024-05-06_182912     8K   344G  18.8G  /
// zroot/ROOT/default                         19.6G   344G  19.1G  /
// zroot/bastille                             7.52G   344G   128K  /usr/local/bastille
// zroot/bastille/backups                     2.74G   344G  2.74G  /usr/local/bastille/backups
// zroot/bastille/cache                         96K   344G    96K  /usr/local/bastille/cache
// zroot/bastille/jails                       4.64G   344G   104K  /usr/local/bastille/jails
// zroot/bastille/jails/photoprism2           3.11G   344G   120K  /usr/local/bastille/jails/photoprism2
// zroot/bastille/jails/photoprism2/root      3.11G   344G  3.11G  /usr/local/bastille/jails/photoprism2/root
// zroot/bastille/jails/rust                  1.53G   344G   120K  /usr/local/bastille/jails/rust
// zroot/bastille/jails/rust/root             1.53G   344G  1.53G  /usr/local/bastille/jails/rust/root
// zroot/bastille/logs                          96K   344G    96K  /var/log/bastille
// zroot/bastille/releases                     148M   344G    96K  /usr/local/bastille/releases
// zroot/bastille/releases/Debian12            148M   344G   148M  /usr/local/bastille/releases/Debian12
// zroot/bastille/templates                     96K   344G    96K  /usr/local/bastille/templates
// zroot/datas                                63.8G   344G  63.8G  /datas
// zroot/home                                 14.0G   344G  14.0G  /home
// zroot/tmp                                  14.9M   344G  14.9M  /tmp
// zroot/usr                                  4.73G   344G    96K  /usr
// zroot/usr/ports                            3.93G   344G  3.93G  /usr/ports
// zroot/usr/src                               816M   344G   816M  /usr/src
// zroot/var                                  10.3M   344G    96K  /var
// zroot/var/audit                              96K   344G    96K  /var/audit
// zroot/var/crash                              96K   344G    96K  /var/crash
// zroot/var/log                              5.57M   344G  5.57M  /var/log
// zroot/var/mail                             3.77M   344G  3.77M  /var/mail
// zroot/var/tmp                               648K   344G   648K  /var/tmp


    let pools = zfs_pools();
    fill_properties(pools)

}

fn fill_properties(pools: Vec<HashMap<String, String>>) -> Vec<serde_json::Value> {

    let mut pools_prop = Vec::new();

    for pool in pools.iter() {
        // TODO get datasets
        let datasets = zfs_datasets(pool.get("name").unwrap().clone());
        let datasets_pros = fill_properties_dataset(datasets);
        pools_prop.push(serde_json::json!([
            {
                "key": "type",
                "value": "zpool"
            },
            {
                "key": "name",
                "value": pool.get("name")
            },
            {
                "key": "partitions",
                "type": "list",
                "value": ["part_id1","part_id2"]
            },
            {
                "key": "totalsize",
                "type": "integer",
                "unit": pool.get("totalsize_unit"),
                "value": pool.get("totalsize")
            },
            {
                "key": "allocatedsize",
                "type": "integer",
                "unit": pool.get("allocatedsize_unit"),
                "value": pool.get("allocatedsize")
            },
            {
                "key": "freesize",
                "type": "integer",
                "unit": pool.get("freesize_unit"),
                "value": pool.get("freesize")
            },
            {
                "key": "allocatedpercentage",
                "type": "integer",
                "unit": "%",
                "value": pool.get("allocatedpercentage")
            },
            {
                "key": "health",
                "value": pool.get("health") // from list: ONLINE, DEGRADED, FAULTED, OFFLINE, REMOVED, UNAVAIL
            },
            {
                "key": "_children",
                "value": datasets_pros
            }
        ]));
    }

    // TODO children, get dataset

    return pools_prop;
}

fn zfs_pools() -> Vec<HashMap<String, String>> {
    let cmd = "zpool";
    let args = ["list", "-L", "-P", "-v"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("zpool list -L -P -v command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(_) => empty,
    };

    // parsing the command output to extract information needed
    let mut pools: Vec<HashMap<String, String>> = Vec::new();
    let mut pool_attr: HashMap<String, String> = HashMap::new();

    // let mut incr: i32 = 0;
    // let mut start_config: bool = false;
    let partitions: Vec<String> = Vec::new();
    let re: Regex = Regex::new(r"^(\s*)(\S+)(?:\s+)(\d+)(\w){1}(?:\s+)(\d+)(\w){1}(?:\s+)(\d+)(\w){1}(?:\s+)(\S+)(?:\s+)(\S+)(?:\s+)(\S+)(?:\s+)([\d\.]+)%(?:\s+)(\S+)(?:\s+)(\S+)").unwrap();
    for line in data.lines() {
        if let Some(mat) = re.captures(line) {
            if pools.iter().count() > 0 {
                pools.push(pool_attr.clone());

                let pool_attr: HashMap<String, String> = HashMap::new();
            }
            if &mat[1] == "" {
                // TODO get data of pool
                pool_attr.insert(String::from("name"), mat[2].trim().to_string());
                pool_attr.insert(String::from("totalsize"), mat[3].to_string());
                pool_attr.insert(String::from("totalsize_unit"), mat[4].to_string());
                pool_attr.insert(String::from("allocatedsize"), mat[5].to_string());
                pool_attr.insert(String::from("allocatedsize_unit"), mat[6].to_string());
                pool_attr.insert(String::from("freesize"), mat[7].to_string());
                pool_attr.insert(String::from("freesize_unit"), mat[8].to_string());
                pool_attr.insert(String::from("allocatedpercentage"), mat[12].to_string());
                pool_attr.insert(String::from("health"), mat[14].to_string());
            } else {
                // TODO now manage partitions

            }
        }
    }
    pools.push(pool_attr);

    return pools;
}

fn zfs_datasets(volume: String) -> Vec<HashMap<String, String>> {
    // zfs list -r zroot

    let cmd = "zfs";
    let args = ["list", "-r", volume.as_str()];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("zfs list -r command error");

    let empty = String::from("");
    let data = match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(_) => empty,
    };

    let mut datasets: Vec<HashMap<String, String>> = Vec::new();
    let mut dataset_attr: HashMap<String, String> = HashMap::new();
    let re: Regex = Regex::new(r"^(\S+)(?:\s+)([\d\.]+)(\w){1}(?:\s+)([\d\.]+)(\w){1}(?:\s+)([\d\.]+)(\w){1}(?:\s+)").unwrap();

    for line in data.lines() {
        if let Some(mat) = re.captures(line) {
            let mut dataset_attr: HashMap<String, String> = HashMap::new();
            dataset_attr.insert(String::from("name"), mat[1].trim().to_string());
            dataset_attr.insert(String::from("type"), "dataset".to_string());
            dataset_attr.insert(String::from("allocatedsize"), mat[2].to_string());
            dataset_attr.insert(String::from("allocatedsize_unit"), mat[3].to_string());
            dataset_attr.insert(String::from("freesize"), mat[4].to_string());
            dataset_attr.insert(String::from("freesize_unit"), mat[5].to_string());
            datasets.push(dataset_attr);
        }
    }
    // TODO fill_properties_dataset
    return datasets;
}

fn fill_properties_dataset(datasets: Vec<HashMap<String, String>>) -> Vec<serde_json::Value> {
    let mut datasets_prop = Vec::new();

    for dataset in datasets.iter() {
        datasets_prop.push(serde_json::json!([
            {
                "key": "type",
                "value": "zdataset"
            },
            {
                "key": "name",
                "value": dataset.get("name")
            },
            {
                "key": "partitions",
                "type": "list",
                "value": []
            },
            {
                "key": "totalsize",
                "type": "integer",
                "unit": "",
                "value": ""
            },
            {
                "key": "allocatedsize",
                "type": "integer",
                "unit": dataset.get("allocatedsize_unit"),
                "value": dataset.get("allocatedsize")
            },
            {
                "key": "freesize",
                "type": "integer",
                "unit": dataset.get("freesize_unit"),
                "value": dataset.get("freesize")
            },
            {
                "key": "allocatedpercentage",
                "type": "integer",
                "unit": "%",
                "value": ""
            },
            {
                "key": "health",
                "value": ""
            },

            ]));
    }
    return datasets_prop;
}

// volume: zpool
//   volume: dataset
//      filesystem

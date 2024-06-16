use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::module;
use crate::common;
use crate::CONFIG;
use crate::CHANNEL;
use crate::{LOCALINVENTORYNEXT, NETWORKDISCOVERYNEXT, NETWORKINVENTORYNEXT, DEPLOYNEXT};
use std::collections::HashMap;

pub fn run_modules_in_thread() {
    let mut running_modules = HashMap::new();

    if CONFIG.localinventory.enabled {
        running_modules.insert(String::from("localinventory"), thread::spawn(|| {
            log::debug!("Run inventory in thread");
            manage_module_executions("localinventory".to_string())
        }));
    }
    if CONFIG.networkdiscovery.enabled {
        running_modules.insert(String::from("networkdiscovery"), thread::spawn(|| {
            log::debug!("Run network discovery in thread");
            manage_module_executions("networkdiscovery".to_string())
        }));
    }
    if CONFIG.networkinventory.enabled {
        running_modules.insert(String::from("networkinventory"), thread::spawn(|| {
            log::debug!("Run network inventory in thread");
            manage_module_executions("networkinventory".to_string())
        }));
    }
    if CONFIG.deploy.enabled {
        running_modules.insert(String::from("deploy"), thread::spawn(|| {
            log::debug!("Run deploy in thread");
            manage_module_executions("deploy".to_string())
        }));
    }

    // Start webserver
    running_modules.insert(String::from("webserver"), thread::spawn(|| {
        log::debug!("Run web server");
        let _ = common::webserver::main();
    }));

    loop {
        let receiver = CHANNEL.1.lock().unwrap();
        let received: String = receiver.recv().unwrap();
        println!("Got: {}", received);
        if running_modules.contains_key(&received) {
            println!("Run module {}", received);
            running_modules.get(&received).unwrap().thread().unpark();
        } else {
            println!("Module {} not running", received);
        }
    }
}

fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn manage_module_executions(module_name: String) {
    let mut next_execution_time = 0;

    loop {
        log::info!("loop iteration for module: {}", module_name);

        log::info!("it's time to run the module: {}", module_name);
        run_module(module_name.clone());

        if module_name == "localinventory".to_string() {
            next_execution_time = get_current_timestamp() + CONFIG.localinventory.contact_time;
            let _ = LOCALINVENTORYNEXT.1.update(next_execution_time);
        };
        if module_name == "networkdiscovery".to_string() {
            next_execution_time = get_current_timestamp() + CONFIG.networkdiscovery.contact_time;
            let _ = NETWORKDISCOVERYNEXT.1.update(next_execution_time);
        };
        if module_name == "networkinventory".to_string() {
            next_execution_time = get_current_timestamp() + CONFIG.networkinventory.contact_time;
            let _ = NETWORKINVENTORYNEXT.1.update(next_execution_time);
        };
        if module_name == "deploy".to_string() {
            next_execution_time = get_current_timestamp() + CONFIG.deploy.contact_time;
            let _ = DEPLOYNEXT.1.update(next_execution_time);
        };
        println!("currenttimestamp: {:?}", get_current_timestamp());
        println!("Next time: {:?}", next_execution_time);
        // We park with timeout as next execution time planned
        thread::park_timeout(Duration::from_secs(next_execution_time - get_current_timestamp()));
        let _ = LOCALINVENTORYNEXT.1.update(0);
    }
}

fn run_module(module_name: String) {
    if module_name == "localinventory" {
        module::localinventory::run_servers::main();
    }
}

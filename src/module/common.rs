use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::module;
use crate::common;
use crate::CONFIG;

pub fn run_modules_in_thread() {
    let mut running_modules = vec![];
    if CONFIG.localinventory.enabled {
        running_modules.push(thread::spawn(|| {
            log::debug!("Run inventory in thread");
            manage_module_executions("localinventory".to_string())
        }));
    }
    if CONFIG.networkdiscovery.enabled {
        running_modules.push(thread::spawn(|| {
            log::debug!("Run network discovery in thread");
            manage_module_executions("networkdiscovery".to_string())
        }));
    }
    if CONFIG.networkinventory.enabled {
        running_modules.push(thread::spawn(|| {
            log::debug!("Run network inventory in thread");
            manage_module_executions("networkinventory".to_string())
        }));
    }
    if CONFIG.deploy.enabled {
        running_modules.push(thread::spawn(|| {
            log::debug!("Run deploy in thread");
            manage_module_executions("deploy".to_string())
        }));
    }

    // Start webserver
    running_modules.push(thread::spawn(|| {
        log::debug!("Run web server");
        let _ = common::webserver::main();
    }));



    // waiting finish threads, prevent finish program if always running
    for running_module in running_modules {
        let _ = running_module.join();
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

        if get_current_timestamp() > next_execution_time {
            log::info!("it's time to run the module: {}", module_name);
            run_module(module_name.clone());

            if module_name == "localinventory".to_string() {
                next_execution_time = get_current_timestamp() + CONFIG.localinventory.contact_time;
            };
            if module_name == "networkdiscovery".to_string() {
                next_execution_time = get_current_timestamp() + CONFIG.networkdiscovery.contact_time;
            };
            if module_name == "networkinventory".to_string() {
                next_execution_time = get_current_timestamp() + CONFIG.networkinventory.contact_time;
            };
            if module_name == "deploy".to_string() {
                next_execution_time = get_current_timestamp() + CONFIG.deploy.contact_time;
            };
            println!("currenttimestamp: {:?}", get_current_timestamp());
            println!("NextTimes: {:?}", next_execution_time);
        }
        // Pause to next execution time
        thread::sleep(Duration::from_secs(next_execution_time - get_current_timestamp()));
    }
}

fn run_module(module_name: String) {
    if module_name == "localinventory" {
        module::localinventory::structure::chassis::run_inventory();
    }
}

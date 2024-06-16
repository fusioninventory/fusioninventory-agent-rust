// file used to read the configuration file

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

// Top level struct to hold the TOML data.
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Data {
    #[serde(default = "g_general_default")]
    pub general: General,
    #[serde(default = "g_network_default")]
    pub network: Network,
    #[serde(default = "g_webinterface_default")]
    pub webinterface: Webinterface,
    #[serde(default = "g_logging_default")]
    pub logging: Logging,
    #[serde(default = "g_localinventory_default")]
    pub localinventory: Localinventory,
    #[serde(default = "g_networkdiscovery_default")]
    pub networkdiscovery: Networkdiscovery,
    #[serde(default = "g_networkinventory_default")]
    pub networkinventory: Networkinventory,
    #[serde(default = "g_deploy_default")]
    pub deploy: Deploy,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct General {
    pub daemon: bool,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Network {
    pub proxy_url: String,
    pub proxy_user: String,
    pub proxy_password: String,
    pub cert_folder: String,
    pub cert_file: String,
    #[serde(default = "ssl_check_default")]
    pub ssl_check: bool,
    #[serde(default = "timeout_default")]
    pub timeout: u64,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Webinterface {
    #[serde(default = "enable_web_default")]
    pub enable_web: bool,
    pub listen_ip: Vec<String>,
    #[serde(default = "port_default")]
    pub port: u16,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Logging {
    #[serde(default = "logger_default")]
    pub logger: String,
    #[serde(default = "logger_level_default")]
    pub logger_level: String,
    pub log_file: String,
    #[serde(default = "maxsize_default")]
    pub maxsize: u64,
    #[serde(default = "logfacility_default")]
    pub logfacility: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Localinventory {
    #[serde(default = "localinventory_enabled_default")]
    pub enabled: bool,
    #[serde(default = "localinventory_contact_time_default")]
    pub contact_time: u64,
    pub servers: Vec<String>,
    pub tags: Vec<String>,
    pub no_types: Vec<String>,
    pub scan_homedirs: bool,
    pub scan_profiles: bool,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Networkdiscovery {
    pub enabled: bool,
    #[serde(default = "networkdiscovery_contact_time_default")]
    pub contact_time: u64,
    pub servers: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Networkinventory {
    pub enabled: bool,
    #[serde(default = "networkinventory_contact_time_default")]
    pub contact_time: u64,
    pub servers: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Deploy {
    pub enabled: bool,
    #[serde(default = "deploy_contact_time_default")]
    pub contact_time: u64,
    pub servers: Vec<String>,
    #[serde(default = "p2p_default")]
    pub p2p: bool,
}

pub fn main() -> Data {
    let filename = "agent.cfg";

    // Read the contents of the config file
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            // load default configuration
            log::warn!("agent.cfg filename not found, load default configuration");
            return Data {
                general: g_general_default(),
                network: g_network_default(),
                webinterface: g_webinterface_default(),
                logging: g_logging_default(),
                localinventory: g_localinventory_default(),
                networkdiscovery: g_networkdiscovery_default(),
                networkinventory: g_networkinventory_default(),
                deploy: g_deploy_default(),
            }
        }
    };

    // load configuration file content
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            println!("Error on load configuration `{:?}`", e);
            exit(1);
        }
    };
    return data;
}

fn ssl_check_default() -> bool {
    true
}

fn timeout_default() -> u64 {
    180
}

fn enable_web_default() -> bool {
    true
}

fn port_default() -> u16 {
    62354
}

fn logger_default() -> String {
    "stderr".to_string()
}

fn logger_level_default() -> String {
    "info".to_string()
}

fn maxsize_default() -> u64 {
    20
}

fn logfacility_default() -> String {
    "LOG_USER".to_string()
}

fn localinventory_enabled_default() -> bool {
    true
}

fn localinventory_contact_time_default() -> u64 {
    3600
}

fn networkdiscovery_contact_time_default() -> u64 {
    604800
}

fn networkinventory_contact_time_default() -> u64 {
    7200
}

fn deploy_contact_time_default() -> u64 {
    1200
}

fn p2p_default() -> bool {
    true
}

fn g_general_default() -> General {
    General {
        daemon: false,
    }
}

fn g_network_default() -> Network {
    Network {
        proxy_url: "".to_string(),
        proxy_user: "".to_string(),
        proxy_password: "".to_string(),
        cert_folder: "".to_string(),
        cert_file: "".to_string(),
        ssl_check: ssl_check_default(),
        timeout: timeout_default(),
    }
}

fn g_webinterface_default() -> Webinterface {
    Webinterface {
        enable_web: enable_web_default(),
        listen_ip: Vec::new(),
        port: port_default(),
    
    }
}

fn g_logging_default() -> Logging {
    Logging {
        logger: logger_default(),
        logger_level: logger_level_default(),
        log_file: "".to_string(),
        maxsize: maxsize_default(),
        logfacility: logfacility_default(),
    }
}

fn g_localinventory_default() -> Localinventory {
    Localinventory {
        enabled: localinventory_enabled_default(),
        contact_time: localinventory_contact_time_default(),
        servers: Vec::new(),
        tags: Vec::new(),
        no_types: Vec::new(),
        scan_homedirs: false,
        scan_profiles: false,
    }
}

fn g_networkdiscovery_default() -> Networkdiscovery {
    Networkdiscovery {
        enabled: false,
        contact_time: networkdiscovery_contact_time_default(),
        servers: Vec::new(),
    }
}

fn g_networkinventory_default() -> Networkinventory {
    Networkinventory {
        enabled: false,
        contact_time: networkinventory_contact_time_default(),
        servers: Vec::new(),
    }
}

fn g_deploy_default() -> Deploy {
    Deploy {
        enabled: false,
        contact_time: deploy_contact_time_default(),
        servers: Vec::new(),
        p2p: p2p_default(),
    }
}


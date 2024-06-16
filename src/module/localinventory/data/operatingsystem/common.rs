use sysinfo::System;
use chrono::Local;

pub fn get_os_name() -> String {
    match System::name() {
        Some(output) => output,
        _ => String::from(""),
    }
}

pub fn get_cpu_arch() -> String {
    match System::cpu_arch() {
        Some(output) => output,
        _ => String::from(""),
    }
}

pub fn get_os_version() -> String {
    match System::long_os_version() {
        Some(output) => output,
        _ => String::from(""),
    }
}

pub fn get_hostname() -> String {
    match System::host_name() {
        Some(output) => output,
        _ => String::from(""),
    }
}

pub fn get_boottime() -> u64 {
    System::boot_time()
}

pub fn get_timezone_name() -> String {
    match iana_time_zone::get_timezone() {
        Ok(name) => name,
        Err(e) => String::from(""),
    }
}

pub fn get_timezone_offset() -> i32 {
    let offset_in_sec = Local::now().offset().local_minus_utc();
    return offset_in_sec;
}

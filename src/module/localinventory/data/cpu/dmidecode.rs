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

use crate::module;

use nparse::*;
use std::process::{Command, Stdio};
use module::localinventory::structure::cpu::FinalProcessorStruct;
use regex::Regex;
use serde::{Deserialize};
use module::localinventory::data::common;

#[derive(Deserialize, Debug)]
struct ProcessorInformation {
    // #[serde(rename = "Socket Designation")]
    // socket_designation: Option<String>,
    // cputype: Option<String>,
    #[serde(rename = "Family")]
    family: Option<String>,
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
    // flags: Vec<String>,
    // version: Option<String>,
    #[serde(rename = "Max Speed")]
    max_speed: Option<String>,
    #[serde(rename = "Serial Number")]
    serial_number: Option<String>,
    #[serde(rename = "Core Count")]
    core_count: Option<String>,
    #[serde(rename = "Thread Count")]
    thread_count: Option<String>,
    // characteristics: Vec<String>,
    id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CacheInformation {
    #[serde(rename = "Socket Designation")]
    socket_designation: Option<String>,
    #[serde(rename = "Configuration")]
    configuration: Option<String>,
    #[serde(rename = "Installed Size")]
    installed_size: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DmidecodeCpu {
    #[serde(rename = "Processor Information")]
    processor_information: Option<ProcessorInformation>,
    #[serde(rename = "Cache Information")]
    cache_information: Option<CacheInformation>,
}

pub fn run_inventory() -> Vec<FinalProcessorStruct> {
    let out: String = run_dmidecode_cmd();
    parse_output(out)
}

fn parse_output(out: String) -> Vec<FinalProcessorStruct> {
    let mut index: i32 = 0;
    let mut processors = Vec::new();

    // Fix for Windows
    let out = out.replace("\r\n", "\n");

    // get each part of dmidecode to identify processor / processors
    let parts = out.split("\n\n");
    for part in parts {
        let result = part.to_string().indent_to_json();
        let result = result.unwrap();
        let frvalue = serde_json::from_value(result);
        if frvalue.is_err() {
            continue;
        }
        let result: DmidecodeCpu = frvalue.unwrap();

        // let result: DmidecodeCpu = serde_json::from_value(result.clone()).unwrap();
        if result.processor_information.is_some() {
            processors.push(parse_processor(out.clone(), result.processor_information.unwrap(), index));
            index = index + 1;
        }
    }
    return processors;
}

fn parse_processor(out: String, processor: ProcessorInformation, index: i32) -> FinalProcessorStruct {
    // get cache
    let rel1: Regex = Regex::new(r"Level 1$").unwrap();
    let rel2: Regex = Regex::new(r"Level 2$").unwrap();
    let rel3: Regex = Regex::new(r"Level 3$").unwrap();
    let re_intunit: Regex = Regex::new(r"^(\d+) (\S+)$").unwrap();

    let mut speed: i32 = 0;
    let mut speed_unit: String = "".to_string();
    let mut l1cache_val: i32 = 0;
    let mut l1cache_unit: String = "".to_string();
    let mut l2cache_val: i32 = 0;
    let mut l2cache_unit: String = "".to_string();
    let mut l3cache_val: i32 = 0;
    let mut l3cache_unit: String = "".to_string();

    let parts = out.split("\n\n");
    for part in parts {
        let result = part.to_string().indent_to_json();
        let result = result.unwrap();
        let frvalue = serde_json::from_value(result);
        if frvalue.is_err() {
            continue;
        }
        let result: DmidecodeCpu = frvalue.unwrap();
        // let result: DmidecodeCpu = serde_json::from_value(result).unwrap();
        if result.cache_information.is_some() {
            let cache: CacheInformation = result.cache_information.unwrap();
            // if cache.configuration.is_some() {
                let configuration = cache.configuration.unwrap_or_default();
                if let Some(mat) = rel1.captures(configuration.as_str()) {
                    if let Some(mat) = re_intunit.captures(cache.installed_size.unwrap_or_default().as_str()) {
                        l1cache_val = mat[1].trim().to_string().parse::<i32>().unwrap();
                        l1cache_unit = mat[2].trim().to_string();
                    }
                } else if let Some(mat) = rel2.captures(configuration.as_str()) {
                    if let Some(mat) = re_intunit.captures(cache.installed_size.unwrap_or_default().as_str()) {
                        l2cache_val = mat[1].trim().to_string().parse::<i32>().unwrap();
                        l2cache_unit = mat[2].trim().to_string();
                    }
                } else if let Some(mat) = rel3.captures(configuration.as_str()) {
                    if let Some(mat) = re_intunit.captures(cache.installed_size.unwrap_or_default().as_str()) {
                        l3cache_val = mat[1].trim().to_string().parse::<i32>().unwrap();
                        l3cache_unit = mat[2].trim().to_string();
                    }
                }
            // }
        }
    }    

    if let Some(mat) = re_intunit.captures(processor.max_speed.unwrap_or_default().as_str()) {
        speed = mat[1].trim().to_string().parse::<i32>().unwrap();
        speed_unit = mat[2].trim().to_string();
    }

    FinalProcessorStruct {
        manufacturer: common::clean_string(processor.manufacturer.unwrap_or_default()),
        id: processor.id.unwrap_or_default(),
        family: common::clean_string(processor.family.unwrap_or_default()),
        speed: speed,
        speed_unit: speed_unit,
        serialnumber: common::clean_serial(processor.serial_number.unwrap_or_default()),
        corecount: processor.core_count.unwrap_or_else(|| "1".to_string()).parse::<i32>().unwrap_or_default(),
        threadcount: processor.thread_count.unwrap_or_default().parse::<i32>().unwrap_or_default(),
        flags: Vec::new(),
        characteristics: Vec::new(),
        l1cache: l1cache_val,
        l1cache_unit: l1cache_unit,
        l2cache: l2cache_val,
        l2cache_unit: l2cache_unit,
        l3cache: l3cache_val,
        l3cache_unit: l3cache_unit,
    }
}

#[cfg(target_os = "windows")]
fn run_dmidecode_cmd() -> String {
    let cmd = "dmidecode.exe";
    let args = ["-qt", "processor", "-t", "cache"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("dmidecode.exe command error");

    // split
    let empty = String::from("");
    match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(_) => empty,
    }
}

#[cfg(not(target_os = "windows"))]
fn run_dmidecode_cmd() -> String {
    let cmd = "dmidecode";
    let args = ["-qt", "processor", "-t", "cache"];

    let output = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .expect("dmidecode command error");

    // split
    let empty = String::from("");
    match String::from_utf8(output.stdout) {
        Ok(x) => x,
        Err(_) => empty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_remi_srv() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/cpu/remi-srv.dat").unwrap();
        let result: Vec<FinalProcessorStruct> = parse_output(data);

        let mut expected = Vec::new();

        expected.push(FinalProcessorStruct {
            manufacturer: "Intel(R) Corporation".to_string(),
            id: "".to_string(),
            family: "Xeon".to_string(),
            speed: 8300,
            speed_unit: "MHz".to_string(),
            serialnumber: "".to_string(),
            corecount: 4,
            threadcount: 4,
            flags: Vec::new(),
            characteristics: Vec::new(),
            l1cache: 256,
            l1cache_unit: "kB".to_string(),
            l2cache: 1,
            l2cache_unit: "MB".to_string(),
            l3cache: 8,
            l3cache_unit: "MB".to_string(),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bi_cpu() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/cpu/bi-cpu.dat").unwrap();
        let result: Vec<FinalProcessorStruct> = parse_output(data);

        let mut expected = Vec::new();

        expected.push(FinalProcessorStruct {
            manufacturer: "Intel".to_string(),
            id: "".to_string(),
            family: "Xeon".to_string(),
            speed: 4800,
            speed_unit: "MHz".to_string(),
            serialnumber: "".to_string(),
            corecount: 6,
            threadcount: 12,
            flags: Vec::new(),
            characteristics: Vec::new(),
            l1cache: 192,
            l1cache_unit: "kB".to_string(),
            l2cache: 1536,
            l2cache_unit: "kB".to_string(),
            l3cache: 12,
            l3cache_unit: "MB".to_string(),
        });
        expected.push(FinalProcessorStruct {
            manufacturer: "Intel".to_string(),
            id: "".to_string(),
            family: "Xeon".to_string(),
            speed: 4800,
            speed_unit: "MHz".to_string(),
            serialnumber: "".to_string(),
            corecount: 6,
            threadcount: 12,
            flags: Vec::new(),
            characteristics: Vec::new(),
            l1cache: 192,
            l1cache_unit: "kB".to_string(),
            l2cache: 1536,
            l2cache_unit: "kB".to_string(),
            l3cache: 12,
            l3cache_unit: "MB".to_string(),
        });        
        assert_eq!(result, expected);

    }

    #[test]
    fn test_parse_amd_zen() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/cpu/thinkpad_ddurieux.dat").unwrap();

        let result: Vec<FinalProcessorStruct> = parse_output(data);

        let mut expected = Vec::new();

        expected.push(FinalProcessorStruct {
            manufacturer: "Advanced Micro Devices, Inc.".to_string(),
            id: "".to_string(),
            family: "Zen".to_string(),
            speed: 4550,
            speed_unit: "MHz".to_string(),
            serialnumber: "".to_string(),
            corecount: 6,
            threadcount: 12,
            flags: Vec::new(),
            characteristics: Vec::new(),
            l1cache: 384,
            l1cache_unit: "kB".to_string(),
            l2cache: 3,
            l2cache_unit: "MB".to_string(),
            l3cache: 16,
            l3cache_unit: "MB".to_string(),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_8cpu_only_l1_cache() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/cpu/SKAVBE01.dat").unwrap();
        let result: Vec<FinalProcessorStruct> = parse_output(data);

        let mut expected = Vec::new();

        for _ in 0..8 {
            expected.push(FinalProcessorStruct {
                manufacturer: "GenuineIntel".to_string(),
                id: "".to_string(),
                family: "".to_string(),
                speed: 30000,
                speed_unit: "MHz".to_string(),
                serialnumber: "".to_string(),
                corecount: 1,
                threadcount: 0,
                flags: Vec::new(),
                characteristics: Vec::new(),
                l1cache: 16,
                l1cache_unit: "kB".to_string(),
                l2cache: 0,
                l2cache_unit: "kB".to_string(),
                l3cache: 0,
                l3cache_unit: "".to_string(),
            });
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_60cpu_no_cache() {
        let data: String = std::fs::read_to_string("tests/localinventory/data/cpu/SFSVBE03.dat").unwrap();
        let result: Vec<FinalProcessorStruct> = parse_output(data);

        let mut expected = Vec::new();

        for _ in 0..2 {
            expected.push(FinalProcessorStruct {
                manufacturer: "GenuineIntel".to_string(),
                id: "".to_string(),
                family: "".to_string(),
                speed: 3139,
                speed_unit: "MHz".to_string(),
                serialnumber: "".to_string(),
                corecount: 1,
                threadcount: 0,
                flags: Vec::new(),
                characteristics: Vec::new(),
                l1cache: 0,
                l1cache_unit: "".to_string(),
                l2cache: 0,
                l2cache_unit: "".to_string(),
                l3cache: 0,
                l3cache_unit: "".to_string(),
            });
        }
        for _ in 0..58 {
            expected.push(FinalProcessorStruct {
                manufacturer: "".to_string(),
                id: "".to_string(),
                family: "".to_string(),
                speed: 3139,
                speed_unit: "MHz".to_string(),
                serialnumber: "".to_string(),
                corecount: 1,
                threadcount: 0,
                flags: Vec::new(),
                characteristics: Vec::new(),
                l1cache: 0,
                l1cache_unit: "".to_string(),
                l2cache: 0,
                l2cache_unit: "".to_string(),
                l3cache: 0,
                l3cache_unit: "".to_string(),
            });
        }
        assert_eq!(result, expected);
    }

}

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

use sysctl::Sysctl;

pub fn run_inventory() -> Vec<serde_json::Value> {
    let xml = load_geom_data_xml();
    match xml {
        Some(x) => {
            return fill_properties(x);
        },
        None => {
            println!("No data");
            return Vec::<serde_json::Value>::new();
        }
    }
}

fn load_geom_data_xml() -> Option<simple_xml::Node> {

    let ctl = match sysctl::Ctl::new("kern.geom.confxml") {
        Ok(f) => f,
        Err(e) => {
            log::error!("Read sysctl kern.geom.confxml have error: {:?}", e);
            return None;
        }
    };

    let val = ctl.value_string().unwrap();
    match simple_xml::from_string(val.as_str()) {
        Ok(f) => Some(f),
        Err(e) => {
            log::error!("Unable to read XML string of kern.geom.confxml: {:?}", e);
            None
        }
    }
}

fn fill_properties(xml: simple_xml::Node) -> Vec<serde_json::Value> {

    // Since there can multiple nodes/tags with the same name, we need to index twice
    // let heading = &xml["heading"][0];
    // println!("XML: {:?}", &xml["class"]);

    let mut disks = Vec::new();

    for i in &xml["class"] {
        if i["name"][0].content == "DISK" {
            for j in &i["geom"] {
                // println!("XML: {:?}", j);

                // <class ref="0xffffffff816b62d0"/>
                // <name>nda0</name>
                // <rank>1</rank>
                // <config></config>
                // <provider id="0xfffff80008bf3c00">
                //     <geom ref="0xfffff80003986e00"/>
                //     <mode>r3w3e7</mode>
                //     <name>nda0</name>
                //     <alias>nvd0</alias>
                //     <mediasize>512110190592</mediasize>
                //     <sectorsize>512</sectorsize>
                //     <stripesize>0</stripesize>
                //     <stripeoffset>0</stripeoffset>
                //     <config>
                //         <fwheads>0</fwheads>
                //         <fwsectors>0</fwsectors>
                //         <rotationrate>0</rotationrate>
                //         <ident>4YC4N027910704S4X</ident>
                //         <lunid>0000000000000000ace42e003a53a7f5</lunid>
                //         <descr>SKHynix_HFS512GEJ4X164N</descr>
                //     </config>
                // </provider>
                let provider = &j["provider"][0];
                let config = &provider["config"][0];

                let data = serde_json::json!([
                    {
                        "key": "name",
                        "value": j["name"][0].content,
                    },
                    {
                        "key": "description",
                        "value": config["descr"][0].content,
                    },
                    {
                        "key": "serialnumber",
                        "value": config["ident"][0].content,
                    },
                    {
                        "key": "size",
                        "type": "integer",
                        "unit": "B",
                        "value": provider["mediasize"][0].content,
                    },
                    {
                        "key": "id",
                        "value": provider.attributes["id"],
                    }
                ]);
                disks.push(data);
            }
        }
    }
    return disks;
}

// camcontrol identify nda0
// pattern => qr/firmware revision[ ]+(\w+)/

extern crate core;

use satisfactory_bp::{next_i64, Resource, parser};
use std::fs::File;
use std::io::Read;
use nom::error::dbg_dmp;

fn main() {
    let mut file = File::open("./Coal Left to Right.sbp").expect("sbp file should exist");

    let mut buf = Vec::new();
    let size = file.read_to_end(&mut buf).unwrap();

    println!("Opened Test.sbp. Size: {size}");

    // let resource_count = i64::from_le_bytes(buf[24..32].try_into().unwrap());
    let resource_count = next_i64(&buf[24..]).unwrap();
    println!("Resource count: {resource_count}");

    let first = &buf[24..];
    println!("{first:x?}");
    
    let (_rest, resources) = parser::resources(first).unwrap();

    // let resource = Resource::try_from(first).unwrap();
    println!("Resource: {resources:?}");
}

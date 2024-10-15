extern crate core;

use flate2::read;
use satisfactory_bp::{next_i64, parser};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./Test.sbp").expect("sbp file should exist");

    let mut buf = Vec::new();
    let size = file.read_to_end(&mut buf).unwrap();

    println!("Opened Test.sbp. Size: {size}");

    // let resource_count = i64::from_le_bytes(buf[24..32].try_into().unwrap());
    let resource_count = next_i64(&buf[24..]).unwrap();
    println!("Resource count: {resource_count}");

    let first = &buf[24..];
    // println!("{first:x?}");

    let (rest, resources) = parser::resources(first).unwrap();

    // let resource = Resource::try_from(first).unwrap();
    println!("Resource: {resources:?}");

    // println!("{:?}", &rest[..32]);

    let (rest, requirements) = parser::requirements(rest).unwrap();
    println!("Requirements: {requirements:?}");

    let (rest, body_header) = parser::body_header(rest).unwrap();
    println!("Body header: {body_header:?}");

    let mut body = Vec::with_capacity(body_header.uncompressed_size as usize);
    let mut decoder = read::ZlibDecoder::new(rest);
    let size = decoder.read_to_end(&mut body).unwrap();
    println!("Uncompressed size: {size}");

    // let mut out_file = File::create("./out.bin").unwrap();
    // let mut z = read::ZlibDecoder::new(&buf[0x208..]);
    // let mut decompressed = Vec::new();
    // let size = z.read_to_end(&mut decompressed).unwrap();
    //
    // println!("Decompressed size: {size}");
    // out_file.write(&decompressed).unwrap();
}

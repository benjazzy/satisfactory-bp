use std::{fs::File, io::Read};

use flate2::read;
use winnow::{Bytes, Parser};

use crate::patterns::{body::blueprint_body, header::header};

mod patterns;

fn main() {
    let mut file = File::open("./blueprints/Test.sbp").expect("sbp file should exist");

    let mut buf = Vec::new();
    let size = file.read_to_end(&mut buf).unwrap();

    println!("Opened Test.sbp. Size: {size}");

    let data: &mut &Bytes = &mut Bytes::new(buf.as_slice());

    let header = header
        .parse_next(data)
        .expect("header parse should succeed");

    println!("Header: {header:?}\n");

    let rest: &[u8] = data;

    let mut compressed_body = Vec::with_capacity(header.body_header.uncompressed_size as usize);
    let mut decoder = read::ZlibDecoder::new(rest);
    let size = decoder.read_to_end(&mut compressed_body).unwrap();

    println!("Read compressed body with size {size}");

    let body = blueprint_body
        .parse(compressed_body.as_slice().into())
        .expect("body parse should succeed");

    println!("Body: {body:?}\n");
}

use crate::bp_write::BPWrite;
use crate::patterns::Blueprint;
use std::io::BufWriter;
use std::{fs::File, io::Read};

mod bp_write;
mod patterns;

fn main() -> color_eyre::Result<()> {
    let blueprint = Blueprint::default();
    let file = File::open("./empty.sbp")?;
    let mut writer = BufWriter::new(file);
    blueprint.bp_write(&mut writer)?;

    // let mut file = File::open("./blueprints/Test.sbp")?;
    // let mut buf = Vec::new();
    // let size = file.read_to_end(&mut buf)?;
    // println!("Opened blueprint file with a size of {size}");
    //
    // let mut body_buffer = Vec::new();
    // let blueprint = Blueprint::new(buf.as_slice(), &mut body_buffer)?;
    //
    // let out_body = File::create("test_body.bin")?;
    // let mut body_writer = BufWriter::new(out_body);
    // blueprint.body.bp_write(&mut body_writer)?;
    //
    // let out_file = File::create("test_out.sbp")?;
    // let mut out_writer = BufWriter::new(out_file);
    // blueprint.bp_write(&mut out_writer)?;

    Ok(())

    //
    // let mut buf = Vec::new();
    // let size = file.read_to_end(&mut buf).unwrap();
    //
    // println!("Opened Test.sbp. Size: {size}");
    //
    // let data: &mut &Bytes = &mut Bytes::new(buf.as_slice());
    //
    // let header = header
    //     .parse_next(data)
    //     .expect("header parse should succeed");
    //
    // println!("Header: {header:#?}\n");
    //
    // let rest: &[u8] = data;
    //
    // let mut compressed_body = Vec::with_capacity(header.body_header.uncompressed_size as usize);
    // let mut decoder = read::ZlibDecoder::new(rest);
    // let size = decoder.read_to_end(&mut compressed_body).unwrap();
    //
    // println!("Read compressed body with size {size}");
    //
    // let body = blueprint_body
    //     .parse(compressed_body.as_slice().into())
    //     .expect("body parse should succeed");
    //
    // println!("Body: {body:#?}\n");
}

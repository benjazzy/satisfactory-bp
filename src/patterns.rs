pub mod body;
pub mod factory_string;
pub mod header;

use body::*;
use color_eyre::eyre::WrapErr;
use factory_string::*;
use flate2::{read, write, Compression};
use header::*;
use std::io::{Error, Read, Write};
use winnow::{Bytes, Parser};
use winnow::stream::AsBytes;
use crate::bp_write::BPWrite;

pub struct Blueprint<'header, 'body> {
    pub header: Header<'header>,
    pub body: BlueprintBody<'body>,
}

impl<'header, 'body> Blueprint<'header, 'body> {
    pub fn new<B: Into<&'header Bytes>>(data: B, body_buffer: &'body mut Vec<u8>) -> color_eyre::Result<Self> {
        let mut data = data.into();
        let header = header
            .parse_next(&mut data).unwrap();
            // .wrap_err("Failed to parse blueprint header")?;

        // let mut compressed_body = Vec::with_capacity(header.body_header.uncompressed_size as usize);
        body_buffer.reserve(header.body_header.uncompressed_size as usize);
        let mut decoder = read::ZlibDecoder::new(data.as_bytes());
        let _ = decoder
            .read_to_end(body_buffer).unwrap();
            // .wrap_err("Failed to decompress blueprint body")?;

        let body = blueprint_body
            .parse(body_buffer.as_slice().into()).unwrap();
            // .wrap_err("Failed to parse blueprint body")?;

        Ok(Blueprint { header, body })
    }
}

impl<W: Write> BPWrite<W> for Blueprint<'_, '_> {
    fn bp_write(mut self, writer: &mut W) -> Result<(), Error> {
        let mut uncompressed_body_bytes = Vec::new();
        self.body.bp_write(&mut uncompressed_body_bytes)?;

        let mut encoder = write::ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&uncompressed_body_bytes)?;
        let compressed_body_bytes = encoder.finish()?;

        self.header.body_header.compressed_size = compressed_body_bytes.len() as u64;
        self.header.body_header.uncompressed_size = uncompressed_body_bytes.len() as u64;

        self.header.bp_write(writer)?;
        writer.write_all(&compressed_body_bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;
    use crate::bp_write::BPWrite;
    use flate2::write::ZlibEncoder;
    use flate2::{Compression, read};
    use winnow::{Bytes, Parser};

    #[test]
    fn check_parse() {
        const DATA: &[u8] = include_bytes!("../blueprints/Test.sbp");

        let data: &mut &Bytes = &mut Bytes::new(DATA);

        let header = header
            .parse_next(data)
            .expect("header parse should succeed");

        let rest: &[u8] = data;

        let mut compressed_body = Vec::with_capacity(header.body_header.uncompressed_size as usize);
        let mut decoder = read::ZlibDecoder::new(rest);
        let size = decoder.read_to_end(&mut compressed_body).unwrap();

        assert_eq!(header.body_header.uncompressed_size as usize, size);

        let body = blueprint_body
            .parse(compressed_body.as_slice().into())
            .expect("body parse should succeed");
        assert_eq!(body.object_headers.len(), 3);
        assert_eq!(body.objects.len(), 3);
    }

    #[test]
    fn check_blueprint() {
        const DATA: &[u8] = include_bytes!("../blueprints/Test.sbp");

        let mut body_buffer = Vec::new();
        let blueprint = Blueprint::new(DATA, &mut body_buffer).expect("Parse should succeed");

        // let mut buf = Vec::new();
        // blueprint.body.bp_write(&mut buf).expect("Body write should succeed");
        // assert_eq!()
        //
        // buf.clear();
        // buf.reserve(DATA.len());
        let mut buf = Vec::with_capacity(DATA.len());
        blueprint.bp_write(&mut buf).expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

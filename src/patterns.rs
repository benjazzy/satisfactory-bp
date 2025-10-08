pub mod body;
pub mod factory_string;
pub mod header;

use crate::bp_write::BPWrite;
use body::*;
use flate2::{Compression, read, write};
use header::*;
use std::io::{Error, Read, Seek, SeekFrom, Write};
use winnow::stream::AsBytes;
use winnow::{Bytes, Parser};

#[derive(Debug, Default)]
pub struct Blueprint<'header> {
    pub header: Header<'header>,
    pub body: BlueprintBody,
}

impl<'header> Blueprint<'header> {
    pub fn new<B: Into<&'header Bytes>>(data: B) -> color_eyre::Result<Self> {
        let mut data = data.into();
        let header = header.parse_next(&mut data).unwrap();
        // .wrap_err("Failed to parse blueprint header")?;

        // let mut compressed_body = Vec::with_capacity(header.body_header.uncompressed_size as usize);
        // body_buffer.reserve(header.body_header.uncompressed_size as usize);
        let mut body_buffer = Vec::new();
        let mut decoder = read::ZlibDecoder::new(data.as_bytes());
        let _ = decoder.read_to_end(&mut body_buffer).unwrap();
        // .wrap_err("Failed to decompress blueprint body")?;

        let body = blueprint_body.parse(body_buffer.as_slice().into()).unwrap();
        // .wrap_err("Failed to parse blueprint body")?;

        Ok(Blueprint { header, body })
    }
}

impl<W: Write + Seek> BPWrite<W> for Blueprint<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), Error> {
        self.header.bp_write(writer)?;

        // Get the position of the start of the compressed and uncompressed body size values
        let body_sizes_pos = writer.stream_position()?;

        // Write dummy values for the header compressed and uncompressed sizes
        writer.write_all(&[0xFF; 32])?;

        let mut uncompressed_body_bytes = Vec::new();
        self.body.bp_write(&mut uncompressed_body_bytes)?;
        let uncompressed_size = uncompressed_body_bytes.len() as u64;

        let mut encoder = write::ZlibEncoder::new(writer, Compression::default());
        encoder.write_all(&uncompressed_body_bytes)?;
        // encoder.flush()?;
        // let compressed_size = encoder.total_out();
        let writer = encoder.finish()?;
        let compressed_size = writer.stream_position()? - body_sizes_pos - 32;

        println!(
            "Finished compressing the body with an uncompressed size of {uncompressed_size} and a compressed size of {compressed_size}"
        );

        writer.seek(SeekFrom::Start(body_sizes_pos))?;
        for _ in 0..2 {
            compressed_size.bp_write(writer)?;
            uncompressed_size.bp_write(writer)?;
        }

        println!("Finished writing the blueprint");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use crate::bp_write::BPWrite;
    use winnow::{Bytes, Parser};

    #[test]
    fn check_parse() {
        const DATA: &[u8] = include_bytes!("../blueprints/Test.sbp");

        let data: &mut &Bytes = &mut Bytes::new(DATA);

        let header = header
            .parse_next(data)
            .expect("header parse should succeed");

        let rest: &[u8] = data;

        // let mut compressed_body = Vec::with_capacity(header.body_header.uncompressed_size as usize);
        // let mut decoder = read::ZlibDecoder::new(rest);
        // let size = decoder.read_to_end(&mut compressed_body).unwrap();
        //
        // assert_eq!(header.body_header.uncompressed_size as usize, size);
        //
        // let body = blueprint_body
        //     .parse(compressed_body.as_slice().into())
        //     .expect("body parse should succeed");
        // assert_eq!(body.object_headers.len(), 3);
        // assert_eq!(body.objects.len(), 3);
    }

    #[test]
    fn check_blueprint() {
        const DATA: &[u8] = include_bytes!("../blueprints/Test.sbp");

        let blueprint = Blueprint::new(DATA).expect("Parse should succeed");

        // let mut buf = Vec::new();
        // blueprint.body.bp_write(&mut buf).expect("Body write should succeed");
        // // assert_eq!()

        let mut buf = Cursor::new(Vec::with_capacity(DATA.len()));
        blueprint.bp_write(&mut buf).expect("Write should succeed");

        assert_eq!(&buf.get_ref()[..471], &DATA[..471]);
    }
}

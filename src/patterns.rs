pub mod body;
pub mod factory_string;
pub mod header;

use body::*;
use factory_string::*;
use header::*;

#[cfg(test)]
mod tests {
    use std::io::Read;

    use flate2::read;
    use winnow::{Bytes, Parser};

    use super::*;

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
}

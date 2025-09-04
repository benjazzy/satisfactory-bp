use std::io::Write;

use winnow::{
    Bytes, Parser,
    binary::{le_u32, le_u64},
    combinator::{alt, fail, seq},
    error::StrContext,
};

use crate::bp_write::BPWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyHeaderVersion {
    V1,
    V2,
}

impl BodyHeaderVersion {
    const V1BYTES: &[u8] = &[0x00; 4];
    const V2BYTES: &[u8] = &[0x22; 4];
}

impl<W: Write> BPWrite<W> for BodyHeaderVersion {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::V1 => Self::V1BYTES.bp_write(writer),
            Self::V2 => Self::V2BYTES.bp_write(writer),
        }
    }
}

pub fn body_header_version(data: &mut &Bytes) -> winnow::Result<BodyHeaderVersion> {
    alt((
        BodyHeaderVersion::V1BYTES
            .map(|_| BodyHeaderVersion::V1)
            .context(StrContext::Label("Version 1")),
        BodyHeaderVersion::V2BYTES
            .map(|_| BodyHeaderVersion::V2)
            .context(StrContext::Label("Version 2")),
        fail.context(StrContext::Label("Did not match V1 or V2")),
    ))
    .parse_next(data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BodyHeader {
    pub header_version: BodyHeaderVersion,
    pub max_chunk_size: u32,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}

impl BodyHeader {
    const MAGIC_NUM: u32 = 0x9E2A83C1;
}

impl<W: Write> BPWrite<W> for &BodyHeader {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        BodyHeader::MAGIC_NUM.bp_write(writer)?;
        self.header_version.bp_write(writer)?;
        self.max_chunk_size.bp_write(writer)?;

        // Padding
        [0u8].bp_write(writer)?;
        0x03000000u32.bp_write(writer)?;

        for _ in 0..2 {
            self.compressed_size.bp_write(writer)?;
            self.uncompressed_size.bp_write(writer)?;
        }

        Ok(())
    }
}

pub fn body_header(data: &mut &Bytes) -> winnow::Result<BodyHeader> {
    const MAGIC_NUMBER: &[u8] = BodyHeader::MAGIC_NUM.to_le_bytes().as_slice();
    const PADDING: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x03];

    let body_header = seq! {BodyHeader {
        _: MAGIC_NUMBER.context(StrContext::Label("magic number 0x9E2A83C1")),
        header_version: body_header_version.context(StrContext::Label("header version")),
        max_chunk_size: le_u32.context(StrContext::Label("max chunk size")),
        _: PADDING.context(StrContext::Label("padding")),
        compressed_size: le_u64.context(StrContext::Label("first compressed size")),
        uncompressed_size: le_u64.context(StrContext::Label("first uncompressed size")),
    }}
    .parse_next(data)?;

    let (check_compressed_size, check_uncompressed_size) = (le_u64, le_u64).parse_next(data)?;
    if body_header.compressed_size != check_compressed_size
        || body_header.uncompressed_size != check_uncompressed_size
    {
        todo!("Create error for mismatched size");
    }

    Ok(body_header)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_body_header_version_1() {
        const DATA: &[u8] = &[0x00; 4];
        let version = body_header_version(&mut DATA.into()).expect("Parse should succeed");

        assert_eq!(version, BodyHeaderVersion::V1)
    }

    #[test]
    fn check_body_header_version_2() {
        const DATA: &[u8] = &[0x22; 4];
        let version = body_header_version(&mut DATA.into()).expect("Parse should succeed");

        assert_eq!(version, BodyHeaderVersion::V2)
    }

    #[test]
    fn check_body_header_version_fail() {
        const DATA: &[u8] = &[0x11; 4];
        let _error = body_header_version(&mut DATA.into()).expect_err("Parse should FAIL");
    }

    #[test]
    fn check_body_header() {
        const DATA: [u8; 0x31] = [
            0xC1, 0x83, 0x2A, 0x9E, 0x22, 0x22, 0x22, 0x22, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x4D, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xD5, 0x0A, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x4D, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xD5,
            0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        const CORRECT: BodyHeader = BodyHeader {
            header_version: BodyHeaderVersion::V2,
            max_chunk_size: 131072,
            compressed_size: 589,
            uncompressed_size: 2773,
        };

        let body_header = body_header
            .parse((&DATA[..]).into())
            .expect("Parse should succeed");

        assert_eq!(body_header, CORRECT);

        let mut buf = Vec::new();
        body_header
            .bp_write(&mut buf)
            .expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

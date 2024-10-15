use crate::types::{BodyHeader, HeaderVersion};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::{le_i32, le_i64, le_u8};
use nom::sequence::pair;
use nom::IResult;

pub fn body_header(blueprint: &[u8]) -> IResult<&[u8], BodyHeader> {
    let (rest, _) = tag(0x9E2A83C1_u32.to_le_bytes())(blueprint)?;
    let (rest, header_version) = map(le_i32, |version| match version {
        0x00000000 => HeaderVersion::V1,
        0x22222222 => HeaderVersion::V2,
        _ => panic!("Implement proper error handling please"),
    })(rest)?;
    let (rest, _) = tag((128_i64 * 1024_i64).to_le_bytes())(rest)?;
    let (rest, compression_algorithm) = le_u8(rest)?;
    let (rest, (compressed_size, uncompressed_size)) = pair(le_i64, le_i64)(rest)?;
    let (rest, (second_compressed_size, second_uncompressed_size)) = pair(le_i64, le_i64)(rest)?;

    if compressed_size != second_compressed_size {
        todo!("Handle corrupted or incorrect compressed size")
    } else if uncompressed_size != second_uncompressed_size {
        todo!("Handle corrupted or incorrect uncompressed size")
    }

    Ok((
        rest,
        BodyHeader::new(
            header_version,
            compression_algorithm,
            compressed_size,
            uncompressed_size,
        ),
    ))
}

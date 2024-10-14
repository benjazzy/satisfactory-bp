use crate::error::FgStringError;
use crate::parser::fg_string;
use nom::bytes::complete::tag;
use nom::multi::{length_count, many0};
use nom::number::complete::le_u64;
use nom::sequence::terminated;
use nom::IResult;

pub fn requirements(blueprint: &[u8]) -> IResult<&[u8], Vec<&str>, FgStringError<&[u8]>> {
    length_count(le_u64, terminated(fg_string, many0(tag(&[0]))))(blueprint)
}

use nom::bytes::complete::tag;
use crate::error::FgStringError;
use crate::parser::fg_string;
use crate::Resource;
use nom::combinator::map;
use nom::multi::length_count;
use nom::number::complete::{le_i32, le_u64};
use nom::sequence::{pair, terminated};
use nom::IResult;

pub fn resource(blueprint: &[u8]) -> IResult<&[u8], Resource, FgStringError<&[u8]>> {
    map(pair(fg_string, le_i32), |(path, amount)| {
        Resource::new(path, amount)
    })(blueprint)
}

pub fn resources(blueprint: &[u8]) -> IResult<&[u8], Vec<Resource>, FgStringError<&[u8]>> {
    terminated(length_count(le_u64, resource), tag(0u32.to_le_bytes()))(blueprint)
}
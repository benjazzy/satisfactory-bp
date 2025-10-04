use std::io::Write;

use winnow::{
    Bytes, Parser,
    binary::{le_u8, le_u32},
    combinator::{alt, preceded, seq},
    error::StrContext,
};

use crate::{
    bp_write::BPWrite,
    patterns::factory_string::{FString, fstring},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteType<'d> {
    Byte(u8),
    FString(FString<'d>),
}

impl<W: Write> BPWrite<W> for &ByteType<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        match self {
            ByteType::Byte(b) => b.bp_write(writer),
            ByteType::FString(s) => s.bp_write(writer),
        }
    }
}

fn byte_type<'d>(data: &mut &Bytes) -> winnow::Result<ByteType<'d>> {
    preceded(
        fstring.verify(|s| s.content == "None\0"),
        preceded(&[0u8], le_u8.map(ByteType::Byte)),
    )
    .parse_next(data)
}

fn fstring_type<'d>(data: &mut &'d Bytes) -> winnow::Result<ByteType<'d>> {
    unimplemented!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteProperty<'d> {
    index: u32,
    value: ByteType<'d>,
}

impl ByteProperty<'_> {
    pub fn size(&self) -> u32 {
        match self.value {
            ByteType::Byte(_) => 19,
            ByteType::FString(fstring) => fstring.size(),
        }
    }
}

impl<W: Write> BPWrite<W> for &ByteProperty<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        let size = match self.value {
            ByteType::Byte(_) => 1,
            ByteType::FString(fstring) => fstring.size(),
        };
        size.bp_write(writer)?;
        self.index.bp_write(writer)?;

        match self.value {
            ByteType::Byte(b) => {
                FString::new("None\0").bp_write(writer)?;
                0u8.bp_write(writer)?;
                b.bp_write(writer)?;
            }
            ByteType::FString(_) => unimplemented!(),
        }

        Ok(())
    }
}

pub fn byte_property<'d>(data: &mut &'d Bytes) -> winnow::Result<ByteProperty<'d>> {
    seq! { ByteProperty {
        _: le_u32.context(StrContext::Label("size")),
        index: le_u32.context(StrContext::Label("index")),
        value: alt((
            byte_type.context(StrContext::Label("byte type")),
            fstring_type.context(StrContext::Label("string type")),
        ))
    }}
    .parse_next(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_byte_property() {
        const DATA: [u8; 0x13] = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4E, 0x6F,
            0x6E, 0x65, 0x00, 0x00, 0xFF,
        ];

        let property = byte_property
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");

        assert_eq!(property.index, 0);
        assert_eq!(property.value, ByteType::Byte(0xFF));
        assert_eq!(property.size() as usize, DATA.len());

        let mut buf = Vec::new();
        property.bp_write(&mut buf).expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

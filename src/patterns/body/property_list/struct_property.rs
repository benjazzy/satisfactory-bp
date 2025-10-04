use std::io::Write;

use winnow::{
    Bytes, Parser,
    binary::{le_f32, le_u32},
    combinator::{dispatch, seq, terminated},
    error::StrContext,
};

use crate::{
    bp_write::BPWrite,
    patterns::{
        body::property_list::{PropertyList, property_list},
        factory_string::{FString, fstring},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum StructDataType<'d> {
    LinearColor(LinearColor),
    Other {
        name: FString<'d>,
        list: PropertyList<'d>,
    },
}

impl StructDataType<'_> {
    const LC: FString<'static> = FString::new("LinearColor\0");

    pub fn size(&self) -> u32 {
        match self {
            StructDataType::LinearColor(_) => 16,
            StructDataType::Other { name, list } => name.size() + list.size() + 16,
        }
    }
}

impl<W: Write> BPWrite<W> for &StructDataType<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        match self {
            StructDataType::LinearColor(linear_color) => {
                StructDataType::LC.bp_write(writer)?;
                [0; 17].bp_write(writer)?;
                linear_color.bp_write(writer)?;
            }
            StructDataType::Other { name, list } => {
                name.bp_write(writer)?;
                [0; 17].bp_write(writer)?;
                list.bp_write(writer)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<W: Write> BPWrite<W> for &LinearColor {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        self.r.bp_write(writer)?;
        self.g.bp_write(writer)?;
        self.b.bp_write(writer)?;
        self.a.bp_write(writer)
    }
}

pub fn linear_color(data: &mut &Bytes) -> winnow::Result<LinearColor> {
    seq! {LinearColor {
        r: le_f32.context(StrContext::Label("red")),
        g: le_f32.context(StrContext::Label("green")),
        b: le_f32.context(StrContext::Label("blue")),
        a: le_f32.context(StrContext::Label("alpha")),
    }}
    .parse_next(data)
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructProperty<'d> {
    pub index: u32,
    pub data: StructDataType<'d>,
}

impl StructProperty<'_> {
    pub fn size(&self) -> u32 {
        self.data.size() + 25
    }
}

impl<W: Write> BPWrite<W> for &StructProperty<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        self.data.size().bp_write(writer)?;
        self.index.bp_write(writer)?;
        self.data.bp_write(writer)
    }
}

pub fn struct_property<'d>(data: &mut &'d Bytes) -> winnow::Result<StructProperty<'d>> {
    seq! {StructProperty {
        _: le_u32,
        index: le_u32.context(StrContext::Label("struct property index")),
        data: dispatch! {terminated(fstring, &[0; 17]);
            StructDataType::LC => linear_color.map(StructDataType::LinearColor).context(StrContext::Label("linear color data")),
            name => property_list.map(|list| StructDataType::Other { name, list }).context(StrContext::Label("property list data")),
        },
    }}.parse_next(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_linear_color() {
        const DATA: [u8; 0x10] = [
            0x3F, 0x2D, 0x79, 0x3C, 0x64, 0xEF, 0x0E, 0x3F, 0xEF, 0x90, 0x10, 0x3F, 0x00, 0x00,
            0x80, 0x3F,
        ];

        let color = linear_color
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");

        assert_eq!(color.r.floor(), 0.0);
        assert_eq!(color.g.floor(), 0.0);
        assert_eq!(color.b.floor(), 0.0);
        assert_eq!(color.a.floor(), 1.0);
    }

    #[test]
    fn check_struct_color() {
        const DATA: [u8; 0x39] = [
            0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x4C, 0x69,
            0x6E, 0x65, 0x61, 0x72, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3F,
            0x2D, 0x79, 0x3C, 0x64, 0xEF, 0x0E, 0x3F, 0xEF, 0x90, 0x10, 0x3F, 0x00, 0x00, 0x80,
            0x3F,
        ];

        let prop = struct_property
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");

        assert_eq!(prop.index, 0);
        assert!(matches!(prop.data, StructDataType::LinearColor(_)));

        let mut buf = Vec::new();
        prop.bp_write(&mut buf).expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

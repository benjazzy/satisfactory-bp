use std::io::Write;

use winnow::{Bytes, Parser, binary::le_u32, combinator::seq, error::StrContext};

use crate::{
    bp_write::BPWrite,
    patterns::factory_string::{FStringExt, fstring},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectProperty {
    pub index: u32,
    pub reference: String,
}

impl ObjectProperty {
    pub fn size(&self) -> u32 {
        self.reference.size() + 13
    }
}

impl<W: Write> BPWrite<W> for &ObjectProperty {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        let size = self.reference.size() + 4;
        size.bp_write(writer)?;
        self.index.bp_write(writer)?;
        [0u8; 5].bp_write(writer)?;
        self.reference.bp_write(writer)
    }
}

pub fn object_property<'d>(data: &mut &'d Bytes) -> winnow::Result<ObjectProperty> {
    seq! { ObjectProperty {
        _: le_u32,
        index: le_u32.context(StrContext::Label("object index")),
        _: &[0; 5],
        reference: fstring.context(StrContext::Label("object reference")).map(ToOwned::to_owned),
    }}
    .parse_next(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_object_property() {
        const DATA: [u8; 0x67] = [
            0x5E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56,
            0x00, 0x00, 0x00, 0x2F, 0x47, 0x61, 0x6D, 0x65, 0x2F, 0x46, 0x61, 0x63, 0x74, 0x6F,
            0x72, 0x79, 0x47, 0x61, 0x6D, 0x65, 0x2F, 0x50, 0x72, 0x6F, 0x74, 0x6F, 0x74, 0x79,
            0x70, 0x65, 0x2F, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x61, 0x62, 0x6C, 0x65, 0x2F, 0x42,
            0x65, 0x61, 0x6D, 0x73, 0x2F, 0x52, 0x65, 0x63, 0x69, 0x70, 0x65, 0x5F, 0x42, 0x65,
            0x61, 0x6D, 0x5F, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x65, 0x64, 0x2E, 0x52, 0x65, 0x63,
            0x69, 0x70, 0x65, 0x5F, 0x42, 0x65, 0x61, 0x6D, 0x5F, 0x50, 0x61, 0x69, 0x6E, 0x74,
            0x65, 0x64, 0x5F, 0x43, 0x00,
        ];

        let prop = object_property
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");

        assert_eq!(prop.index, 0);
        assert_eq!(
            prop.reference,
            "/Game/FactoryGame/Prototype/Buildable/Beams/Recipe_Beam_Painted.Recipe_Beam_Painted_C\0",
        );
        assert_eq!(prop.size() as usize, DATA.len());

        let mut buf = Vec::new();
        prop.bp_write(&mut buf).expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

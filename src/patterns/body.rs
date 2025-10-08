mod object;
mod object_header;
mod object_ref;
mod property_list;

use crate::bp_write::BPWrite;
pub use object::*;
pub use object_header::*;
pub use object_ref::*;
pub use property_list::*;
use std::io::{Error, Write};
use winnow::{
    Bytes, Parser,
    binary::le_u32,
    combinator::{repeat, seq},
    error::StrContext,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlueprintBody {
    pub object_headers: Vec<ObjectHeaderType>,
    pub objects: Vec<ObjectType>,
}

pub fn blueprint_body(data: &mut &Bytes) -> winnow::Result<BlueprintBody> {
    seq! {BlueprintBody {
        _: le_u32.context(StrContext::Label("body size")),
        _: le_u32.context(StrContext::Label("object headers size")),
        object_headers: le_u32.context(StrContext::Label("object headers count"))
            .flat_map(
                |count| repeat(count as usize, object_header_type)
            ).context(StrContext::Label("object headers")),
        _: le_u32.context(StrContext::Label("objects size")),
        objects: le_u32.context(StrContext::Label("object headers count"))
            .flat_map(
                |count| repeat(count as usize, actor_object.map(ObjectType::Actor))
            ).context(StrContext::Label("objects")),
    }}
    .parse_next(data)
}

impl<W: Write> BPWrite<W> for &BlueprintBody {
    fn bp_write(self, writer: &mut W) -> Result<(), Error> {
        // Size includes count
        let headers_size: u32 = self
            .object_headers
            .iter()
            .map(|h| dbg!(h.size()))
            .sum::<u32>()
            + 4;
        let headers_count: u32 = self
            .object_headers
            .len()
            .try_into()
            .expect("Headers too long");

        // Size includes count
        let objects_size: u32 = self.objects.iter().map(ObjectType::size).sum::<u32>() + 4;
        let objects_count: u32 = self.objects.len().try_into().expect("Objects too long");

        let size = headers_size + objects_size + 8;

        size.bp_write(writer)?;

        headers_size.bp_write(writer)?;
        headers_count.bp_write(writer)?;
        for header in &self.object_headers {
            header.bp_write(writer)?;
        }

        objects_size.bp_write(writer)?;
        objects_count.bp_write(writer)?;
        for object in &self.objects {
            object.bp_write(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_blueprint_body() {
        const DATA: &[u8] = include_bytes!("../../blueprints/Test-uncompressed.bin");

        let body = blueprint_body
            .parse(DATA.into())
            .expect("parse should succeed");

        let mut buf = Vec::new();
        body.bp_write(&mut buf).expect("write should succeed");
        assert_eq!(buf, DATA);
    }
}

mod object;
mod object_header;
mod object_ref;
mod property_list;

pub use object::*;
pub use object_header::*;
pub use object_ref::*;
pub use property_list::*;
use winnow::{
    Bytes, Parser,
    binary::le_u32,
    combinator::{repeat, seq},
    error::StrContext,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BlueprintBody<'d> {
    pub object_headers: Vec<ObjectHeaderType<'d>>,
    pub objects: Vec<ObjectType<'d>>,
}

pub fn blueprint_body<'d>(data: &mut &'d Bytes) -> winnow::Result<BlueprintBody<'d>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_blueprint_body() {
        const DATA: &[u8] = include_bytes!("../../blueprints/Test-uncompressed.bin");

        let body = blueprint_body
            .parse(DATA.into())
            .expect("parse should succeed");
    }
}

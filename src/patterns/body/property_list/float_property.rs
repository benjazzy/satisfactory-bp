use winnow::{
    Bytes, Parser,
    binary::{le_f32, le_u32},
    combinator::seq,
    error::StrContext,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatProperty {
    pub index: u32,
    pub value: f32,
}

pub fn float_property(data: &mut &Bytes) -> winnow::Result<FloatProperty> {
    seq! {FloatProperty {
        _: le_u32,
        index: le_u32.context(StrContext::Label("float index")),
        _: &[0],
        value: le_f32.context(StrContext::Label("float value")),
    }}
    .parse_next(data)
}

#[cfg(test)]
mod tests {
    use crate::patterns::body::property_list::float_property;

    use super::*;

    #[test]
    fn check_float_property() {
        const DATA: [u8; 0x0D] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC8, 0x42,
        ];
        let prop = float_property
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");
        assert_eq!(prop.index, 0);
        assert_eq!(prop.value, 100.0);
    }
}

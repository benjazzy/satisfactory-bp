mod byte_property;
mod float_property;
mod object_property;

use winnow::{
    Bytes, Parser,
    combinator::{alt, dispatch, empty, fail, seq},
    error::StrContext,
};

pub use byte_property::*;
pub use float_property::*;
pub use object_property::*;

use crate::patterns::factory_string::{FString, fstring};

#[derive(Debug, Clone, PartialEq)]
enum PropertyType<'d> {
    ByteProperty(ByteProperty<'d>),
    FloatProperty(FloatProperty),
    ObjectProperty,
    StructProperty,
    None,
}

#[derive(Debug, Clone, PartialEq)]
struct Property<'d> {
    pub name: FString<'d>,
    pub property: PropertyType<'d>,
}

fn none_property<'d>(data: &mut &'d Bytes) -> winnow::Result<Property<'d>> {
    const NP: FString = FString::new("None\0");
    seq! { Property {
        name: fstring.verify(|s| *s == NP).context(StrContext::Label("name")),
        property: empty.value(PropertyType::None).context(StrContext::Label("property")),
    }}
    .parse_next(data)
}

fn property<'d>(data: &mut &'d Bytes) -> winnow::Result<Property<'d>> {
    const BP: FString = FString::new("ByteProperty\0");
    const FP: FString = FString::new("FloatProperty\0");

    alt((
        none_property.context(StrContext::Label("none property")),
        seq! {Property {
            name: fstring.context(StrContext::Label("property name")),
            property: dispatch! {fstring.context(StrContext::Label("property type"));
                BP => byte_property.map(PropertyType::ByteProperty),
                FP => float_property.map(PropertyType::FloatProperty),
                _ => fail.context(StrContext::Label("unkown property")),
            }
        }},
    ))
    .parse_next(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_property_type_byte() {
        const DATA: [u8; 0x33] = [
            0x0B, 0x00, 0x00, 0x00, 0x6D, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x53, 0x6C, 0x6F, 0x74,
            0x00, 0x0D, 0x00, 0x00, 0x00, 0x42, 0x79, 0x74, 0x65, 0x50, 0x72, 0x6F, 0x70, 0x65,
            0x72, 0x74, 0x79, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x4E, 0x6F, 0x6E, 0x65, 0x00, 0x00, 0xFF,
        ];

        let prop = property
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");

        assert_eq!(prop.name.content, "mColorSlot\0");
        assert!(matches!(prop.property, PropertyType::ByteProperty(_)));
    }

    #[test]
    fn check_property_type_none() {
        const DATA: [u8; 0x09] = [0x05, 0x00, 0x00, 0x00, 0x4E, 0x6F, 0x6E, 0x65, 0x00];

        let prop = property
            .parse(DATA.as_slice().into())
            .expect("Parse should succeed");

        assert_eq!(prop.name.content, "None\0");
        assert_eq!(prop.property, PropertyType::None);
    }
}

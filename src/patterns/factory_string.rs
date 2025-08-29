use std::ops::Deref;

use winnow::error::{ContextError, ParserError, StrContext};

use winnow::token::take;

use winnow::binary::le_u32;
use winnow::{Bytes, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FString<'s> {
    pub length: u32,
    pub content: &'s str,
}

impl<'s> FString<'s> {
    pub const fn new(value: &'s str) -> Self {
        FString {
            length: value.len() as u32,
            content: value,
        }
    }
}

pub fn fstring<'d>(data: &mut &'d Bytes) -> winnow::Result<FString<'d>> {
    let length = le_u32
        .context(StrContext::Label("string length"))
        .parse_next(data)?;
    let content = take(length)
        .context(StrContext::Label("string content"))
        .parse_next(data)?;
    let content = str::from_utf8(content).map_err(|_| ContextError::from_input(data))?;

    Ok(FString { length, content })
}

impl<'d> Deref for FString<'d> {
    type Target = str;

    fn deref(&self) -> &'d Self::Target {
        self.content
    }
}

impl<'d> From<&'d str> for FString<'d> {
    fn from(value: &'d str) -> Self {
        FString::new(value)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[test]
    fn check_fstring() {
        const DATA: [u8; 0x52] = [
            0x4E, 0x00, 0x00, 0x00, 0x2F, 0x47, 0x61, 0x6D, 0x65, 0x2F, 0x46, 0x61, 0x63, 0x74,
            0x6F, 0x72, 0x79, 0x47, 0x61, 0x6D, 0x65, 0x2F, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72,
            0x63, 0x65, 0x2F, 0x50, 0x61, 0x72, 0x74, 0x73, 0x2F, 0x53, 0x74, 0x65, 0x65, 0x6C,
            0x50, 0x6C, 0x61, 0x74, 0x65, 0x2F, 0x44, 0x65, 0x73, 0x63, 0x5F, 0x53, 0x74, 0x65,
            0x65, 0x6C, 0x50, 0x6C, 0x61, 0x74, 0x65, 0x2E, 0x44, 0x65, 0x73, 0x63, 0x5F, 0x53,
            0x74, 0x65, 0x65, 0x6C, 0x50, 0x6C, 0x61, 0x74, 0x65, 0x5F, 0x43, 0x00,
        ];
        const STRING: &str =
            "/Game/FactoryGame/Resource/Parts/SteelPlate/Desc_SteelPlate.Desc_SteelPlate_C\0";

        let factory_string = fstring(&mut Bytes::new(&DATA[..])).expect("Parser should succeed");
        assert_eq!(factory_string.length as usize, DATA.len() - 4);
        assert_eq!(factory_string.content, STRING);
    }
}

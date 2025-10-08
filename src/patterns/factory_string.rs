use std::io::Write;
use std::ops::Index;
use std::slice::SliceIndex;

use winnow::error::{ContextError, ParserError, StrContext};

use winnow::token::take;

use winnow::binary::le_u32;
use winnow::{Bytes, Parser};

use crate::bp_write::BPWrite;

pub trait FStringExt {
    fn size(&self) -> u32;
}

impl FStringExt for str {
    fn size(&self) -> u32 {
        (self.len() + 4)
            .try_into()
            .expect("Factory string too long")
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct FStr {
    pub content: str,
}

impl FStr {
    pub const fn new(value: &str) -> &FStr {
        // SAFETY: FStr is just a wrapper around str,
        // therefore converting &str to &FStr is safe.
        // std::path::Path uses this method.
        unsafe { &*(value as *const str as *const FStr) }
    }

    pub const fn len(&self) -> usize {
        self.content.len()
    }

    pub fn size(&self) -> u32 {
        (self.len() + 4)
            .try_into()
            .expect("Factory string too long")
    }
}

impl<W: Write> BPWrite<W> for &FStr {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        let len: u32 = self.len().try_into().expect("Factory String is too long");

        writer.write_all(len.to_le_bytes().as_slice())?;
        writer.write_all(self.content.as_bytes())
    }
}

pub fn fstring<'d>(data: &mut &'d Bytes) -> winnow::Result<&'d FStr> {
    let length = le_u32
        .context(StrContext::Label("string length"))
        .parse_next(data)?;
    let content = take(length)
        .context(StrContext::Label("string content"))
        .parse_next(data)?;
    let content = str::from_utf8(content).map_err(|_| ContextError::from_input(data))?;

    Ok(FStr::new(content))
}

impl AsRef<str> for FStr {
    fn as_ref(&self) -> &str {
        &self.content
    }
}

impl<'d> From<&'d str> for &'d FStr {
    fn from(value: &'d str) -> Self {
        FStr::new(value)
    }
}

impl PartialEq<str> for FStr {
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}

impl<I> Index<I> for FStr
where
    I: SliceIndex<str>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_ref().index(index)
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
        assert_eq!(factory_string.len(), DATA.len() - 4);
        assert_eq!(&factory_string.content, STRING);

        let mut buf = Vec::new();
        factory_string
            .bp_write(&mut buf)
            .expect("Serialization should succeed");

        assert_eq!(buf, DATA);
    }
}

use std::io::Write;

use winnow::{Bytes, Parser, combinator::seq, error::StrContext};

use crate::{
    bp_write::BPWrite,
    patterns::factory_string::{FStringExt, fstring},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectRef<'d> {
    pub level_name: &'d str,
    pub path_name: &'d str,
}

impl ObjectRef<'_> {
    pub fn size(&self) -> u32 {
        self.level_name.size() + self.path_name.size()
    }
}

impl<W: Write> BPWrite<W> for &ObjectRef<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        self.level_name.bp_write(writer)?;
        self.path_name.bp_write(writer)
    }
}

pub fn object_ref<'d>(data: &mut &'d Bytes) -> winnow::Result<ObjectRef<'d>> {
    seq! {ObjectRef {
        level_name: fstring.context(StrContext::Label("level name")),
        path_name: fstring.context(StrContext::Label("path name")),
    }}
    .parse_next(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_object_ref() {
        const DATA: [u8; 0x4D] = [
            0x11, 0x00, 0x00, 0x00, 0x50, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6E, 0x74,
            0x5F, 0x4C, 0x65, 0x76, 0x65, 0x6C, 0x00, 0x34, 0x00, 0x00, 0x00, 0x50, 0x65, 0x72,
            0x73, 0x69, 0x73, 0x74, 0x65, 0x6E, 0x74, 0x5F, 0x4C, 0x65, 0x76, 0x65, 0x6C, 0x3A,
            0x50, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6E, 0x74, 0x4C, 0x65, 0x76, 0x65,
            0x6C, 0x2E, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x61, 0x62, 0x6C, 0x65, 0x53, 0x75, 0x62,
            0x73, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x00,
        ];

        let level = "Persistent_Level\0";
        let path = "Persistent_Level:PersistentLevel.BuildableSubsystem\0";

        let object_ref = object_ref
            .parse((&DATA[..]).into())
            .expect("Parse should succeed");

        assert_eq!(object_ref.level_name, level);
        assert_eq!(object_ref.path_name, path);

        let mut buf = Vec::new();
        object_ref.bp_write(&mut buf).expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

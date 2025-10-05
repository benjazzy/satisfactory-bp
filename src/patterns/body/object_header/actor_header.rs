use std::io::Write;

use winnow::{
    Bytes, Parser,
    binary::{le_f32, le_u32},
    combinator::seq,
    error::StrContext,
};

use crate::{
    bp_write::BPWrite,
    patterns::factory_string::{FString, fstring},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActorHeader<'d> {
    pub type_path: FString<'d>,
    pub root_object: FString<'d>,
    pub instance_name: FString<'d>,
    pub unknown: u32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub rotation_w: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
}

impl ActorHeader<'_> {
    pub fn size(&self) -> u32 {
        let type_path_size = self.type_path.size();
        let root_object_size = self.root_object.size();
        let instance_name_size = self.instance_name.size();

        type_path_size + root_object_size + instance_name_size + 48
    }
}

impl<W: Write> BPWrite<W> for &ActorHeader<'_> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        self.type_path.bp_write(writer)?;
        self.root_object.bp_write(writer)?;
        self.instance_name.bp_write(writer)?;
        self.unknown.bp_write(writer)?;
        self.rotation_x.bp_write(writer)?;
        self.rotation_y.bp_write(writer)?;
        self.rotation_z.bp_write(writer)?;
        self.rotation_w.bp_write(writer)?;
        self.position_x.bp_write(writer)?;
        self.position_y.bp_write(writer)?;
        self.position_z.bp_write(writer)?;
        self.scale_x.bp_write(writer)?;
        self.scale_y.bp_write(writer)?;
        self.scale_z.bp_write(writer)?;

        [0u8; 4].bp_write(writer)
    }
}

pub fn actor_header<'d>(data: &mut &'d Bytes) -> winnow::Result<ActorHeader<'d>> {
    seq! { ActorHeader {
        type_path: fstring.context(StrContext::Label("type path")),
        root_object: fstring.context(StrContext::Label("root_object")),
        instance_name: fstring.context(StrContext::Label("instance_name")),
        unknown: le_u32.context(StrContext::Label("unknown")),

        rotation_x: le_f32.context(StrContext::Label("rotation x")),
        rotation_y: le_f32.context(StrContext::Label("rotation y")),
        rotation_z: le_f32.context(StrContext::Label("rotation z")),
        rotation_w: le_f32.context(StrContext::Label("rotation w")),

        position_x: le_f32.context(StrContext::Label("position x")),
        position_y: le_f32.context(StrContext::Label("position y")),
        position_z: le_f32.context(StrContext::Label("position z")),

        scale_x: le_f32.context(StrContext::Label("scale x")),
        scale_y: le_f32.context(StrContext::Label("scale y")),
        scale_z: le_f32.context(StrContext::Label("scale z")),

        _: &[0; 4],
    }}
    .parse_next(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_actor_header() {
        const DATA: [u8; 0xE2] = [
            0x54, 0x00, 0x00, 0x00, 0x2F, 0x47, 0x61, 0x6D, 0x65, 0x2F, 0x46, 0x61, 0x63, 0x74,
            0x6F, 0x72, 0x79, 0x47, 0x61, 0x6D, 0x65, 0x2F, 0x50, 0x72, 0x6F, 0x74, 0x6F, 0x74,
            0x79, 0x70, 0x65, 0x2F, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x61, 0x62, 0x6C, 0x65, 0x2F,
            0x42, 0x65, 0x61, 0x6D, 0x73, 0x2F, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x5F, 0x42, 0x65,
            0x61, 0x6D, 0x5F, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x65, 0x64, 0x2E, 0x42, 0x75, 0x69,
            0x6C, 0x64, 0x5F, 0x42, 0x65, 0x61, 0x6D, 0x5F, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x65,
            0x64, 0x5F, 0x43, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x65, 0x72, 0x73, 0x69, 0x73,
            0x74, 0x65, 0x6E, 0x74, 0x5F, 0x4C, 0x65, 0x76, 0x65, 0x6C, 0x00, 0x41, 0x00, 0x00,
            0x00, 0x50, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6E, 0x74, 0x5F, 0x4C, 0x65,
            0x76, 0x65, 0x6C, 0x3A, 0x50, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6E, 0x74,
            0x4C, 0x65, 0x76, 0x65, 0x6C, 0x2E, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x5F, 0x42, 0x65,
            0x61, 0x6D, 0x5F, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x65, 0x64, 0x5F, 0x43, 0x5F, 0x32,
            0x31, 0x34, 0x35, 0x33, 0x39, 0x31, 0x38, 0x31, 0x39, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF3, 0x04, 0x35, 0xBF, 0xF3, 0x04,
            0x35, 0x3F, 0x00, 0x00, 0x48, 0x43, 0x00, 0x00, 0x96, 0xC4, 0x00, 0x00, 0x48, 0x43,
            0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00,
            0x00, 0x00,
        ];

        let correct = ActorHeader {
            type_path: "/Game/FactoryGame/Prototype/Buildable/Beams/Build_Beam_Painted.Build_Beam_Painted_C\0".into(),
            root_object: "Persistent_Level\0".into(),
            instance_name: "Persistent_Level:PersistentLevel.Build_Beam_Painted_C_2145391819\0".into(),
            unknown: 1,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: -std::f32::consts::FRAC_1_SQRT_2,
            rotation_w: std::f32::consts::FRAC_1_SQRT_2,
            position_x: 200.0,
            position_y: -1200.0,
            position_z: 200.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
        };

        let actor_header = actor_header
            .parse((&DATA[..]).into())
            .expect("Parse should succeed");

        assert_eq!(actor_header, correct);

        assert_eq!(actor_header.size() as usize, DATA.len());

        let mut buf = Vec::new();
        actor_header
            .bp_write(&mut buf)
            .expect("Write should succeed");

        assert_eq!(buf, DATA);
    }
}

use winnow::{Bytes, Parser, binary::le_f32, combinator::seq, error::StrContext};

use crate::patterns::factory_string::fstring;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
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
}

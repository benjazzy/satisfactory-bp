use crate::error::FgStringError;
use nom::character::complete::char;
use nom::combinator::{map, map_res};
use nom::multi::length_data;
use nom::number::complete::le_u32;
use nom::sequence::terminated;
use nom::IResult;

pub fn fg_string(bytes: &[u8]) -> IResult<&[u8], &str, FgStringError<&[u8]>> {
    map_res(
        terminated(
            length_data(map(le_u32, |length| length - 1)),
            char('\0'),
        ),
        |b| std::str::from_utf8(b),
    )(bytes)
}

#[cfg(test)]
mod tests {
    use crate::parser::base_types::fg_string;

    const TEST_BLUEPRINT: &[u8] = include_bytes!("../../Test.sbp");

    #[test]
    fn check_fg_string() {
        const RESOURCE_PATH: &str =
            "/Game/FactoryGame/Resource/Parts/SteelPlate/Desc_SteelPlate.Desc_SteelPlate_C";
        let (rest, path) = fg_string(&TEST_BLUEPRINT[0x20..]).expect("Input should be valid");
        // let path = std::str::from_utf8(path).expect("String should be valid utf8");

        assert_eq!(path, RESOURCE_PATH);
        assert_eq!(rest[0], 0x02);
    }
}

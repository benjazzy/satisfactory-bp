use crate::error::FgStringError;
use crate::parser::fg_string;
use crate::types::Resource;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::length_count;
use nom::number::complete::{le_i32, le_u64};
use nom::sequence::{pair, terminated};
use nom::IResult;

pub fn resource(blueprint: &[u8]) -> IResult<&[u8], Resource, FgStringError<&[u8]>> {
    map(pair(fg_string, le_i32), |(path, amount)| {
        Resource::new(path, amount)
    })(blueprint)
}

pub fn resources(blueprint: &[u8]) -> IResult<&[u8], Vec<Resource>, FgStringError<&[u8]>> {
    map(
        pair(
            length_count(
                map(le_u64, |l| l - 1),
                terminated(resource, tag(0u32.to_le_bytes())),
            ), //All but the last resource has a 0u32 between it and the next resource.
            resource,
        ),
        |(mut list, single)| {
            list.push(single);
            list
        },
    )(blueprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BLUEPRINT: &[u8] = include_bytes!("../../Test.sbp");

    #[test]
    fn check_resources() {
        let test_bytes = &TEST_BLUEPRINT[24..284];
        let test_resources = &[
            Resource::new(
                "/Game/FactoryGame/Resource/Parts/SteelPlate/Desc_SteelPlate.Desc_SteelPlate_C",
                2,
            ),
            Resource::new(
                "/Game/FactoryGame/Resource/Parts/IronPlate/Desc_IronPlate.Desc_IronPlate_C",
                2,
            ),
            Resource::new(
                "/Game/FactoryGame/Resource/Parts/Cement/Desc_Cement.Desc_Cement_C",
                2,
            ),
        ];

        let (rest, resources) = resources(test_bytes).expect("Test bytes should be valid");
        assert_eq!(rest, &[2u8]);
        assert_eq!(resources, test_resources,);
    }

    #[test]
    fn check_resource() {
        let test_bytes = &TEST_BLUEPRINT[32..118];

        let (rest, resource) = resource(test_bytes).expect("Test input should be valid");
        assert_eq!(rest, &[]);
        assert_eq!(
            resource,
            Resource::new(
                "/Game/FactoryGame/Resource/Parts/SteelPlate/Desc_SteelPlate.Desc_SteelPlate_C",
                2
            )
        )
    }
}

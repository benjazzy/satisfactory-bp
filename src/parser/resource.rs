use crate::error::FgStringError;
use crate::parser::fg_string;
use crate::Resource;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::length_count;
use nom::number::complete::{le_i32, le_i64, le_u64};
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
    use nom::AsBytes;

    const TEST_BLUEPRINT: &[u8] = include_bytes!("../../Test.sbp");

    fn gen_resource(path: &str, count: i32) -> Vec<u8> {
        let mut resource = Vec::from((path.len() + 1).to_le_bytes());
        resource.extend_from_slice(path.as_bytes());
        resource.push(b'\0');
        resource.extend_from_slice(&count.to_le_bytes());

        resource
    }

    fn gen_resource_list(resources: &[Vec<u8>]) -> Vec<u8> {
        let mut list = Vec::from(&resources.len().to_le_bytes());
        let iter = resources.iter().flatten();
        list.extend(iter);

        list
    }

    #[test]
    fn check_resources() {
        let test_bytes = gen_resource_list(&[gen_resource("/1", 1), gen_resource("/2", 2)]);

        let (rest, resources) = resources(dbg!(&test_bytes)).expect("Test bytes should be valid");
        assert!(rest.is_empty());
        assert_eq!(
            resources,
            vec![Resource::new("/1", 1), Resource::new("/2", 2)]
        );
    }

    #[test]
    fn check_resources_test_file() {
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
    fn check_resource_test_file() {
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

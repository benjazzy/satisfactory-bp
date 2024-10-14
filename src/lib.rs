mod error;
pub mod parser;

use std::str::Utf8Error;

pub fn next_i64(bytes: &[u8]) -> Option<i64> {
    if bytes.len() < 8 {
        return None;
    }

    let slice = &bytes[..8];
    let num = i64::from_le_bytes(
        slice
            .try_into()
            .expect("Slice should be of the correct size"),
    );

    Some(num)
}

pub fn next_i32(bytes: &[u8]) -> Option<i32> {
    if bytes.len() < 4 {
        return None;
    }

    let slice = &bytes[..4];
    let num = i32::from_le_bytes(
        slice
            .try_into()
            .expect("Slice should be of the correct size"),
    );

    Some(num)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Resource<'a> {
    resource_path: &'a str,
    count: i32,
}

impl<'a> Resource<'a> {
    pub fn new(resource_path: &'a str, count: i32) -> Self {
        Resource {
            resource_path,
            count,
        }
    }

    pub fn next_resource(bytes: &'a [u8]) -> Option<(Self, &[u8])> {
        let size = Self::resource_size(bytes)? as usize;
        let resource = Self::try_from(bytes).ok()?;

        Some((resource, &bytes[size..]))
    }

    // pub fn collect(bytes: &'a [u8], count: usize) -> (Vec<Self>, &[u8]) {
    //     let mut iter = ResourceIter(bytes);
    //     let resources = iter.take(count).collect::<Vec<_>>();
    //
    //     (resources, iter.0)
    // }

    pub fn resource_size(bytes: &[u8]) -> Option<i32> {
        Some(next_i32(bytes)? + 8)
    }
}

impl<'a> TryFrom<&'a [u8]> for Resource<'a> {
    type Error = Utf8Error;

    fn try_from(value: &'a [u8]) -> Result<Resource<'a>, Self::Error> {
        let resource_path = std::str::from_utf8(&value[4..value.len() - 9])?;
        let count = i32::from_le_bytes(
            value[value.len() - 8..]
                .try_into()
                .expect("Should be 8 bytes"),
        );

        Ok(Resource {
            resource_path,
            count,
        })
    }
}

pub struct ResourceIter<'a>(&'a [u8]);

impl<'a> Iterator for ResourceIter<'a> {
    type Item = Resource<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (resource, next) = Resource::next_resource(self.0)?;
        self.0 = next;

        Some(resource)
    }
}

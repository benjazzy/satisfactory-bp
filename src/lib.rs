mod error;
pub mod parser;
pub mod types;

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

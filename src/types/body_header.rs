#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HeaderVersion {
    V1,
    V2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BodyHeader {
    header_version: HeaderVersion,
    compression_algorithm: u8,
    compressed_size: i64,
    uncompressed_size: i64,
}

impl BodyHeader {
    pub fn new(
        header_version: HeaderVersion,
        compression_algorithm: u8,
        compressed_size: i64,
        uncompressed_size: i64,
    ) -> Self {
        BodyHeader {
            header_version,
            compression_algorithm,
            compressed_size,
            uncompressed_size,
        }
    }
}

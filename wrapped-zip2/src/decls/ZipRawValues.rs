macro_rules! ZipRawValues {
    () => {
        pub (crate) struct ZipRawValues { pub (crate) crc32 : u32 , pub (crate) compressed_size : u64 , pub (crate) uncompressed_size : u64 , }
    };
}

ZipRawValues!()
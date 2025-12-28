macro_rules! crc32_braid {
    () => {
        pub fn crc32_braid (start : u32 , buf : & [u8]) -> u32 { braid :: crc32_braid :: < 5 > (start , buf) }
    };
}

crc32_braid!()
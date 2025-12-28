macro_rules! BitWriter {
    () => {
        struct BitWriter < 'a > { pub (crate) pending : Pending < 'a > , pub (crate) bit_buffer : u64 , pub (crate) bits_used : u8 , # [doc = " total bit length of compressed file (NOTE: zlib-ng uses a 32-bit integer here)"] # [cfg (feature = "ZLIB_DEBUG")] compressed_len : usize , # [doc = " bit length of compressed data sent (NOTE: zlib-ng uses a 32-bit integer here)"] # [cfg (feature = "ZLIB_DEBUG")] bits_sent : usize , }
    };
}

BitWriter!();
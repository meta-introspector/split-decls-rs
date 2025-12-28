macro_rules! naive_adler32 {
    () => {
        # [cfg (test)] fn naive_adler32 (start_checksum : u32 , data : & [u8]) -> u32 { const MOD_ADLER : u32 = 65521 ; let mut a = start_checksum & 0xFFFF ; let mut b = (start_checksum >> 16) & 0xFFFF ; for & byte in data { a = (a + byte as u32) % MOD_ADLER ; b = (b + a) % MOD_ADLER ; } (b << 16) | a }
    };
}

naive_adler32!();
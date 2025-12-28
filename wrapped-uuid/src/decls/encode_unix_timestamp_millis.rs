macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! encode_unix_timestamp_millis {
    () => {
        deps!();
        pub (crate) const fn encode_unix_timestamp_millis (millis : u64 , counter_random_bytes : & [u8 ; 10] ,) -> Uuid { let millis_high = ((millis >> 16) & 0xFFFF_FFFF) as u32 ; let millis_low = (millis & 0xFFFF) as u16 ; let counter_random_version = (counter_random_bytes [1] as u16 | ((counter_random_bytes [0] as u16) << 8) & 0x0FFF) | (0x7 << 12) ; let mut d4 = [0 ; 8] ; d4 [0] = (counter_random_bytes [2] & 0x3F) | 0x80 ; d4 [1] = counter_random_bytes [3] ; d4 [2] = counter_random_bytes [4] ; d4 [3] = counter_random_bytes [5] ; d4 [4] = counter_random_bytes [6] ; d4 [5] = counter_random_bytes [7] ; d4 [6] = counter_random_bytes [8] ; d4 [7] = counter_random_bytes [9] ; Uuid :: from_fields (millis_high , millis_low , counter_random_version , & d4) }
    };
}

encode_unix_timestamp_millis!();
macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! encode_gregorian_timestamp {
    () => {
        deps!();
        pub (crate) const fn encode_gregorian_timestamp (ticks : u64 , counter : u16 , node_id : & [u8 ; 6] ,) -> Uuid { let time_low = (ticks & 0xFFFF_FFFF) as u32 ; let time_mid = ((ticks >> 32) & 0xFFFF) as u16 ; let time_high_and_version = (((ticks >> 48) & 0x0FFF) as u16) | (1 << 12) ; let mut d4 = [0 ; 8] ; d4 [0] = (((counter & 0x3F00) >> 8) as u8) | 0x80 ; d4 [1] = (counter & 0xFF) as u8 ; d4 [2] = node_id [0] ; d4 [3] = node_id [1] ; d4 [4] = node_id [2] ; d4 [5] = node_id [3] ; d4 [6] = node_id [4] ; d4 [7] = node_id [5] ; Uuid :: from_fields (time_low , time_mid , time_high_and_version , & d4) }
    };
}

encode_gregorian_timestamp!()
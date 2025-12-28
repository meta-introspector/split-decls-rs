macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! decode_sorted_gregorian_timestamp {
    () => {
        deps!();
        pub (crate) const fn decode_sorted_gregorian_timestamp (uuid : & Uuid) -> (u64 , u16) { let bytes = uuid . as_bytes () ; let ticks : u64 = ((bytes [0]) as u64) << 52 | (bytes [1] as u64) << 44 | (bytes [2] as u64) << 36 | (bytes [3] as u64) << 28 | (bytes [4] as u64) << 20 | (bytes [5] as u64) << 12 | ((bytes [6] & 0xF) as u64) << 8 | (bytes [7] as u64) ; let counter : u16 = ((bytes [8] & 0x3F) as u16) << 8 | (bytes [9] as u16) ; (ticks , counter) }
    };
}

decode_sorted_gregorian_timestamp!()
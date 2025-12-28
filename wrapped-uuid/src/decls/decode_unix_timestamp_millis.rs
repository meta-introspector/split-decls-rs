macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! decode_unix_timestamp_millis {
    () => {
        deps!();
        pub (crate) const fn decode_unix_timestamp_millis (uuid : & Uuid) -> u64 { let bytes = uuid . as_bytes () ; let millis : u64 = (bytes [0] as u64) << 40 | (bytes [1] as u64) << 32 | (bytes [2] as u64) << 24 | (bytes [3] as u64) << 16 | (bytes [4] as u64) << 8 | (bytes [5] as u64) ; millis }
    };
}

decode_unix_timestamp_millis!()
macro_rules! encode_offset {
    () => {
        fn encode_offset (len : u32) -> (u8 , u32 , usize) { let log = len . ilog2 () ; let lower = len & ((1 << log) - 1) ; (log as u8 , lower , log as usize) }
    };
}

encode_offset!()
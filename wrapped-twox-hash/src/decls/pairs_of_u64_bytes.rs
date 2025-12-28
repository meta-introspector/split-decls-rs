macro_rules! pairs_of_u64_bytes {
    () => {
        # [inline] # [cfg (feature = "xxhash3_128")] pub fn pairs_of_u64_bytes (bytes : & [u8]) -> & [[[u8 ; 16] ; 2]] { let (u64_bytes , _) = bytes . bp_as_chunks :: < 16 > () ; let (pairs , _) = u64_bytes . bp_as_chunks :: < 2 > () ; pairs }
    };
}

pairs_of_u64_bytes!();
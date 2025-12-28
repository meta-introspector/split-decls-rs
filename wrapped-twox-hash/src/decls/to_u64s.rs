macro_rules! to_u64s {
    () => {
        # [inline] pub fn to_u64s (bytes : & [u8 ; 16]) -> [u64 ; 2] { let (pair , _) = bytes . bp_as_chunks :: < 8 > () ; [pair [0] , pair [1]] . map (u64 :: from_le_bytes) }
    };
}

to_u64s!();
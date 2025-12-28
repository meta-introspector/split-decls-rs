macro_rules! max_leb128_len {
    () => {
        # [doc = " Returns the length of the longest LEB128 encoding for `T`, assuming `T` is an integer type"] pub const fn max_leb128_len < T > () -> usize { (size_of :: < T > () * 8) . div_ceil (7) }
    };
}

max_leb128_len!();
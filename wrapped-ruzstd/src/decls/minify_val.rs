macro_rules! minify_val {
    () => {
        # [doc = " Returns the same value, but represented using the smallest number of bytes needed."] # [doc = " Returned vector will be 1, 2, 4, or 8 bytes in length. Zero is represented as 1 byte."] # [doc = ""] # [doc = " Operates in **little-endian**."] pub fn minify_val (val : u64) -> Vec < u8 > { let new_size = find_min_size (val) ; val . to_le_bytes () [0 .. new_size] . to_vec () }
    };
}

minify_val!();
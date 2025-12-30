// Generated macro for impl_1_to_3_bytes_combined (function)
macro_rules! Depcrate_xxhash3impl_1_to_3_bytes_combined {
() => {
// Module: crate::xxhash3
// Provides: {"impl_1_to_3_bytes_combined"}
// Dependencies: {}
# [inline (always)] pub fn impl_1_to_3_bytes_combined (input : & [u8]) -> u32 { assert_input_range ! (1 ..= 3 , input . len ()) ; let input_length = input . len () as u8 ; input [input . len () - 1] . into_u32 () | input_length . into_u32 () << 8 | input [0] . into_u32 () << 16 | input [input . len () >> 1] . into_u32 () << 24 }
};
}

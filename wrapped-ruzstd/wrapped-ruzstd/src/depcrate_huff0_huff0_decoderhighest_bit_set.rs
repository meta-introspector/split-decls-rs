// Generated macro for highest_bit_set (function)
macro_rules! Depcrate_huff0_huff0_decoderhighest_bit_set {
() => {
// Module: crate::huff0::huff0_decoder
// Provides: {"highest_bit_set"}
// Dependencies: {}
# [doc = " Assert that the provided value is greater than zero, and returns the"] # [doc = " 32 - the number of leading zeros"] fn highest_bit_set (x : u32) -> u32 { assert ! (x > 0) ; u32 :: BITS - x . leading_zeros () }
};
}

// Generated macro for highest_bit_set (function)
macro_rules! Depcrate_fse_fse_decoderhighest_bit_set {
() => {
// Module: crate::fse::fse_decoder
// Provides: {"highest_bit_set"}
// Dependencies: {}
fn highest_bit_set (x : u32) -> u32 { assert ! (x > 0) ; u32 :: BITS - x . leading_zeros () }
};
}

// Generated macro for max_leb128_len (function)
macro_rules! Depcrate_leb128max_leb128_len {
() => {
// Module: crate::leb128
// Provides: {"max_leb128_len"}
// Dependencies: {}
# [doc = " Returns the length of the longest LEB128 encoding for `T`, assuming `T` is an integer type"] pub const fn max_leb128_len < T > () -> usize { (size_of :: < T > () * 8) . div_ceil (7) }
};
}

// Generated macro for highest_bit_set (function)
macro_rules! Depcrate_huff0_huff0_encoderhighest_bit_set {
() => {
// Module: crate::huff0::huff0_encoder
// Provides: {"highest_bit_set"}
// Dependencies: {}
# [doc = " Assert that the provided value is greater than zero, and returns index of the first set bit"] fn highest_bit_set (x : usize) -> usize { assert ! (x > 0) ; usize :: BITS as usize - x . leading_zeros () as usize }
};
}

// Generated macro for xor (function)
macro_rules! Depcrate_aes_ctrxor {
() => {
// Module: crate::aes_ctr
// Provides: {"xor"}
// Dependencies: {}
# [doc = " XORs a slice in place with another slice."] # [inline] fn xor (dest : & mut [u8] , src : & [u8]) { assert_eq ! (dest . len () , src . len ()) ; for (lhs , rhs) in dest . iter_mut () . zip (src . iter ()) { * lhs ^= * rhs ; } }
};
}

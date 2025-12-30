// Generated macro for compress (macro)
macro_rules! Depcrate_commoncompress {
() => {
// Module: crate::common
// Provides: {"compress"}
// Dependencies: {}
# [doc = " Compression round for SipHash algorithm."] macro_rules ! compress { ($ state : expr) => { { compress ! ($ state . v0 , $ state . v1 , $ state . v2 , $ state . v3) } } ; ($ v0 : expr , $ v1 : expr , $ v2 : expr , $ v3 : expr) => { { $ v0 = $ v0 . wrapping_add ($ v1) ; $ v1 = $ v1 . rotate_left (13) ; $ v1 ^= $ v0 ; $ v0 = $ v0 . rotate_left (32) ; $ v2 = $ v2 . wrapping_add ($ v3) ; $ v3 = $ v3 . rotate_left (16) ; $ v3 ^= $ v2 ; $ v0 = $ v0 . wrapping_add ($ v3) ; $ v3 = $ v3 . rotate_left (21) ; $ v3 ^= $ v0 ; $ v2 = $ v2 . wrapping_add ($ v1) ; $ v1 = $ v1 . rotate_left (17) ; $ v1 ^= $ v2 ; $ v2 = $ v2 . rotate_left (32) ; } } ; }
};
}

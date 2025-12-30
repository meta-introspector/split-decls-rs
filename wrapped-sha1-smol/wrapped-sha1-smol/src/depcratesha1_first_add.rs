// Generated macro for sha1_first_add (function)
macro_rules! Depcratesha1_first_add {
() => {
// Module: crate
// Provides: {"sha1_first_add"}
// Dependencies: {}
# [doc = " Not an intrinsic, but adds a word to the first element of a vector."] # [inline] fn sha1_first_add (e : u32 , w0 : u32x4) -> u32x4 { let u32x4 (a , b , c , d) = w0 ; u32x4 (e . wrapping_add (a) , b , c , d) }
};
}

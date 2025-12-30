// Generated macro for random (function)
macro_rules! Depcrate_runtimerandom {
() => {
// Module: crate::runtime
// Provides: {"random"}
// Dependencies: {}
# [doc = " `getauxval(AT_RANDOM)`—Returns the address of 16 pseudorandom bytes."] # [doc = ""] # [doc = " These bytes are for use by libc. For anything else, use the `rand` crate."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [inline] pub fn random () -> * const [u8 ; 16] { backend :: param :: auxv :: random () }
};
}

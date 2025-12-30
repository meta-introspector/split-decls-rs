// Generated macro for impl_49 (impl)
macro_rules! Depcrate_bytesimpl_49 {
() => {
// Module: crate::bytes
// Provides: {"impl_49"}
// Dependencies: {}
impl Bytes { # [doc = " Wrap an existing `&[u8]`."] pub fn new (bytes : & [u8]) -> & Self { unsafe { & * (bytes as * const [u8] as * const Bytes) } } }
};
}

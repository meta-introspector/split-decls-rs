// Generated macro for impl_460 (impl)
macro_rules! Depcrate_formatimpl_460 {
() => {
// Module: crate::format
// Provides: {"impl_460"}
// Dependencies: {}
impl < 'a > Buf < 'a > { pub fn new (bytes : & 'a mut [u8]) -> Self { Buf { bytes , offset : 0 } } pub fn as_str (& self) -> & str { let slice = & self . bytes [.. self . offset] ; unsafe { str :: from_utf8_unchecked (slice) } } }
};
}

// Generated macro for impl_88 (impl)
macro_rules! Depcrate_dataimpl_88 {
() => {
// Module: crate::data
// Provides: {"impl_88"}
// Dependencies: {}
impl Deref for CtxtStsBuf { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Self as * const u8 , mem :: size_of :: < Self > ()) } } }
};
}

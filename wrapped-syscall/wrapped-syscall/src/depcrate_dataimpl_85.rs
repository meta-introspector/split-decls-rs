// Generated macro for impl_85 (impl)
macro_rules! Depcrate_dataimpl_85 {
() => {
// Module: crate::data
// Provides: {"impl_85"}
// Dependencies: {}
impl Deref for ProcSchemeAttrs { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Self as * const u8 , mem :: size_of :: < Self > ()) } } }
};
}

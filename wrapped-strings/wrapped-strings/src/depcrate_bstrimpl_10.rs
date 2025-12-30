// Generated macro for impl_10 (impl)
macro_rules! Depcrate_bstrimpl_10 {
() => {
// Module: crate::bstr
// Provides: {"impl_10"}
// Dependencies: {}
impl From < & str > for BSTR { fn from (value : & str) -> Self { let value : alloc :: vec :: Vec < u16 > = value . encode_utf16 () . collect () ; Self :: from_wide (& value) } }
};
}

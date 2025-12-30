// Generated macro for impl_32 (impl)
macro_rules! Depcrate_hstringimpl_32 {
() => {
// Module: crate::hstring
// Provides: {"impl_32"}
// Dependencies: {}
impl Drop for HSTRING { fn drop (& mut self) { if let Some (header) = self . as_header () { unsafe { if header . flags & HSTRING_REFERENCE_FLAG == 0 && header . count . release () == 0 { HStringHeader :: free (self . 0) ; } } } } }
};
}

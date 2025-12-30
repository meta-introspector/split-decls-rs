// Generated macro for impl_31 (impl)
macro_rules! Depcrate_hstringimpl_31 {
() => {
// Module: crate::hstring
// Provides: {"impl_31"}
// Dependencies: {}
impl Clone for HSTRING { fn clone (& self) -> Self { if let Some (header) = self . as_header () { Self (header . duplicate ()) } else { Self :: new () } } }
};
}

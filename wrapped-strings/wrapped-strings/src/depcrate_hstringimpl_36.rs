// Generated macro for impl_36 (impl)
macro_rules! Depcrate_hstringimpl_36 {
() => {
// Module: crate::hstring
// Provides: {"impl_36"}
// Dependencies: {}
impl From < & str > for HSTRING { fn from (value : & str) -> Self { unsafe { Self :: from_wide_iter (value . encode_utf16 () , value . len ()) } } }
};
}

// Generated macro for impl_238 (impl)
macro_rules! Depcrate_stream_bstrimpl_238 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_238"}
// Dependencies: {}
impl < 'a > From < & 'a str > for & 'a BStr { # [inline] fn from (s : & 'a str) -> & 'a BStr { BStr :: new (s . as_bytes ()) } }
};
}

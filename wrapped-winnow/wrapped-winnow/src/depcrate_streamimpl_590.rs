// Generated macro for impl_590 (impl)
macro_rules! Depcrate_streamimpl_590 {
() => {
// Module: crate::stream
// Provides: {"impl_590"}
// Dependencies: {}
impl < 's > FindSlice < & 's str > for & [u8] { # [inline (always)] fn find_slice (& self , substr : & 's str) -> Option < core :: ops :: Range < usize > > { self . find_slice (substr . as_bytes ()) } }
};
}

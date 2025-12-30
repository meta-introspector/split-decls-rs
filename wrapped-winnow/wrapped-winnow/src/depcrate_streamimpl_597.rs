// Generated macro for impl_597 (impl)
macro_rules! Depcrate_streamimpl_597 {
() => {
// Module: crate::stream
// Provides: {"impl_597"}
// Dependencies: {}
impl < 's > FindSlice < (& 's str , & 's str , & 's str) > for & str { # [inline (always)] fn find_slice (& self , substr : (& 's str , & 's str , & 's str)) -> Option < core :: ops :: Range < usize > > { self . as_bytes () . find_slice (substr) } }
};
}

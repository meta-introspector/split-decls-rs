// Generated macro for impl_594 (impl)
macro_rules! Depcrate_streamimpl_594 {
() => {
// Module: crate::stream
// Provides: {"impl_594"}
// Dependencies: {}
impl < 's > FindSlice < & 's str > for & str { # [inline (always)] fn find_slice (& self , substr : & 's str) -> Option < core :: ops :: Range < usize > > { self . as_bytes () . find_slice (substr) } }
};
}

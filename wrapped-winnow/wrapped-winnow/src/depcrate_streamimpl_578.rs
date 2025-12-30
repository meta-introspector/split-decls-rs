// Generated macro for impl_578 (impl)
macro_rules! Depcrate_streamimpl_578 {
() => {
// Module: crate::stream
// Provides: {"impl_578"}
// Dependencies: {}
impl < 's > FindSlice < & 's [u8] > for & [u8] { # [inline (always)] fn find_slice (& self , substr : & 's [u8]) -> Option < core :: ops :: Range < usize > > { memmem (self , substr) } }
};
}

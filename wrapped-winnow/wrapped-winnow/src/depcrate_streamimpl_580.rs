// Generated macro for impl_580 (impl)
macro_rules! Depcrate_streamimpl_580 {
() => {
// Module: crate::stream
// Provides: {"impl_580"}
// Dependencies: {}
impl < 's > FindSlice < (& 's [u8] , & 's [u8]) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (& 's [u8] , & 's [u8])) -> Option < core :: ops :: Range < usize > > { memmem2 (self , substr) } }
};
}

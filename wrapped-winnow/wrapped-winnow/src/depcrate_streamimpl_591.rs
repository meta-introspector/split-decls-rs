// Generated macro for impl_591 (impl)
macro_rules! Depcrate_streamimpl_591 {
() => {
// Module: crate::stream
// Provides: {"impl_591"}
// Dependencies: {}
impl < 's > FindSlice < (& 's str ,) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (& 's str ,)) -> Option < core :: ops :: Range < usize > > { memmem (self , substr . 0 . as_bytes ()) } }
};
}

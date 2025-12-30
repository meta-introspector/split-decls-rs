// Generated macro for impl_593 (impl)
macro_rules! Depcrate_streamimpl_593 {
() => {
// Module: crate::stream
// Provides: {"impl_593"}
// Dependencies: {}
impl < 's > FindSlice < (& 's str , & 's str , & 's str) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (& 's str , & 's str , & 's str)) -> Option < core :: ops :: Range < usize > > { memmem3 (self , (substr . 0 . as_bytes () , substr . 1 . as_bytes () , substr . 2 . as_bytes () ,) ,) } }
};
}

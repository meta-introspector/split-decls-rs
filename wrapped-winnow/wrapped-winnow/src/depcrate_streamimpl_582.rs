// Generated macro for impl_582 (impl)
macro_rules! Depcrate_streamimpl_582 {
() => {
// Module: crate::stream
// Provides: {"impl_582"}
// Dependencies: {}
impl FindSlice < char > for & [u8] { # [inline (always)] fn find_slice (& self , substr : char) -> Option < core :: ops :: Range < usize > > { let mut b = [0 ; 4] ; let substr = substr . encode_utf8 (& mut b) ; self . find_slice (& * substr) } }
};
}

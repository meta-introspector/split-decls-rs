// Generated macro for impl_583 (impl)
macro_rules! Depcrate_streamimpl_583 {
() => {
// Module: crate::stream
// Provides: {"impl_583"}
// Dependencies: {}
impl FindSlice < (char ,) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (char ,)) -> Option < core :: ops :: Range < usize > > { let mut b = [0 ; 4] ; let substr0 = substr . 0 . encode_utf8 (& mut b) ; self . find_slice ((& * substr0 ,)) } }
};
}

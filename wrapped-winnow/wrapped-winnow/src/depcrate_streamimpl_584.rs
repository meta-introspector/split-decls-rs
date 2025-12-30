// Generated macro for impl_584 (impl)
macro_rules! Depcrate_streamimpl_584 {
() => {
// Module: crate::stream
// Provides: {"impl_584"}
// Dependencies: {}
impl FindSlice < (char , char) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (char , char)) -> Option < core :: ops :: Range < usize > > { let mut b = [0 ; 4] ; let substr0 = substr . 0 . encode_utf8 (& mut b) ; let mut b = [0 ; 4] ; let substr1 = substr . 1 . encode_utf8 (& mut b) ; self . find_slice ((& * substr0 , & * substr1)) } }
};
}

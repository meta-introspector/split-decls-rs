// Generated macro for impl_587 (impl)
macro_rules! Depcrate_streamimpl_587 {
() => {
// Module: crate::stream
// Provides: {"impl_587"}
// Dependencies: {}
impl FindSlice < (u8 ,) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (u8 ,)) -> Option < core :: ops :: Range < usize > > { memchr (substr . 0 , self) . map (| i | i .. i + 1) } }
};
}

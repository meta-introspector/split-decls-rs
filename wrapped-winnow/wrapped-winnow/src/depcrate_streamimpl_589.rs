// Generated macro for impl_589 (impl)
macro_rules! Depcrate_streamimpl_589 {
() => {
// Module: crate::stream
// Provides: {"impl_589"}
// Dependencies: {}
impl FindSlice < (u8 , u8 , u8) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (u8 , u8 , u8)) -> Option < core :: ops :: Range < usize > > { memchr3 (substr , self) . map (| i | i .. i + 1) } }
};
}

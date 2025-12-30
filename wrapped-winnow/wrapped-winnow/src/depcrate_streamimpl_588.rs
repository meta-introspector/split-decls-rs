// Generated macro for impl_588 (impl)
macro_rules! Depcrate_streamimpl_588 {
() => {
// Module: crate::stream
// Provides: {"impl_588"}
// Dependencies: {}
impl FindSlice < (u8 , u8) > for & [u8] { # [inline (always)] fn find_slice (& self , substr : (u8 , u8)) -> Option < core :: ops :: Range < usize > > { memchr2 (substr , self) . map (| i | i .. i + 1) } }
};
}

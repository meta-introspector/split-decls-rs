// Generated macro for impl_586 (impl)
macro_rules! Depcrate_streamimpl_586 {
() => {
// Module: crate::stream
// Provides: {"impl_586"}
// Dependencies: {}
impl FindSlice < u8 > for & [u8] { # [inline (always)] fn find_slice (& self , substr : u8) -> Option < core :: ops :: Range < usize > > { memchr (substr , self) . map (| i | i .. i + 1) } }
};
}

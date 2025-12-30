// Generated macro for impl_601 (impl)
macro_rules! Depcrate_streamimpl_601 {
() => {
// Module: crate::stream
// Provides: {"impl_601"}
// Dependencies: {}
impl FindSlice < (char , char , char) > for & str { # [inline (always)] fn find_slice (& self , substr : (char , char , char)) -> Option < core :: ops :: Range < usize > > { self . as_bytes () . find_slice (substr) } }
};
}

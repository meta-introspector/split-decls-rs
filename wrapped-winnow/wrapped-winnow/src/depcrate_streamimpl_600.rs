// Generated macro for impl_600 (impl)
macro_rules! Depcrate_streamimpl_600 {
() => {
// Module: crate::stream
// Provides: {"impl_600"}
// Dependencies: {}
impl FindSlice < (char , char) > for & str { # [inline (always)] fn find_slice (& self , substr : (char , char)) -> Option < core :: ops :: Range < usize > > { self . as_bytes () . find_slice (substr) } }
};
}

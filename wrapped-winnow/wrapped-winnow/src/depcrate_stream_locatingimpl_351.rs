// Generated macro for impl_351 (impl)
macro_rules! Depcrate_stream_locatingimpl_351 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_351"}
// Dependencies: {}
impl < I , T > FindSlice < T > for LocatingSlice < I > where I : FindSlice < T > , { # [inline (always)] fn find_slice (& self , substr : T) -> Option < core :: ops :: Range < usize > > { self . input . find_slice (substr) } }
};
}

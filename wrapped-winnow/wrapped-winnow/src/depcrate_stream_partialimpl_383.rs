// Generated macro for impl_383 (impl)
macro_rules! Depcrate_stream_partialimpl_383 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_383"}
// Dependencies: {}
impl < I , T > FindSlice < T > for Partial < I > where I : FindSlice < T > , { # [inline (always)] fn find_slice (& self , substr : T) -> Option < core :: ops :: Range < usize > > { self . input . find_slice (substr) } }
};
}

// Generated macro for impl_382 (impl)
macro_rules! Depcrate_stream_partialimpl_382 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_382"}
// Dependencies: {}
impl < I , T > Compare < T > for Partial < I > where I : Compare < T > , { # [inline (always)] fn compare (& self , t : T) -> CompareResult { self . input . compare (t) } }
};
}

// Generated macro for impl_430 (impl)
macro_rules! Depcrate_stream_recoverableimpl_430 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_430"}
// Dependencies: {}
impl < I , E , T > FindSlice < T > for Recoverable < I , E > where I : Stream , I : FindSlice < T > , { # [inline (always)] fn find_slice (& self , substr : T) -> Option < core :: ops :: Range < usize > > { self . input . find_slice (substr) } }
};
}

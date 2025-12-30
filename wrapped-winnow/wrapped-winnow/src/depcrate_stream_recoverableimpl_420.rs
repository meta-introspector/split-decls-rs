// Generated macro for impl_420 (impl)
macro_rules! Depcrate_stream_recoverableimpl_420 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_420"}
// Dependencies: {}
impl < I , E > SliceLen for Recoverable < I , E > where I : SliceLen , I : Stream , { # [inline (always)] fn slice_len (& self) -> usize { self . input . slice_len () } }
};
}

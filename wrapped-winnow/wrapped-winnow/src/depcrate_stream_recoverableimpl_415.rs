// Generated macro for impl_415 (impl)
macro_rules! Depcrate_stream_recoverableimpl_415 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_415"}
// Dependencies: {}
impl < I , E > Default for Recoverable < I , E > where I : Default + Stream , { # [inline] fn default () -> Self { Self :: new (I :: default ()) } }
};
}

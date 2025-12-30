// Generated macro for impl_428 (impl)
macro_rules! Depcrate_stream_recoverableimpl_428 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_428"}
// Dependencies: {}
impl < I , E > AsBStr for Recoverable < I , E > where I : Stream , I : AsBStr , { # [inline (always)] fn as_bstr (& self) -> & [u8] { self . input . as_bstr () } }
};
}

// Generated macro for impl_427 (impl)
macro_rules! Depcrate_stream_recoverableimpl_427 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_427"}
// Dependencies: {}
impl < I , E > AsBytes for Recoverable < I , E > where I : Stream , I : AsBytes , { # [inline (always)] fn as_bytes (& self) -> & [u8] { self . input . as_bytes () } }
};
}

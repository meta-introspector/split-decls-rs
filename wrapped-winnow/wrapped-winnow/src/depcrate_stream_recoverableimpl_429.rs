// Generated macro for impl_429 (impl)
macro_rules! Depcrate_stream_recoverableimpl_429 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_429"}
// Dependencies: {}
impl < I , E , U > Compare < U > for Recoverable < I , E > where I : Stream , I : Compare < U > , { # [inline (always)] fn compare (& self , other : U) -> CompareResult { self . input . compare (other) } }
};
}

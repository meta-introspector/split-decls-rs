// Generated macro for impl_422 (impl)
macro_rules! Depcrate_stream_recoverableimpl_422 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_422"}
// Dependencies: {}
impl < I , E > Location for Recoverable < I , E > where I : Location , I : Stream , { # [inline (always)] fn previous_token_end (& self) -> usize { self . input . previous_token_end () } # [inline (always)] fn current_token_start (& self) -> usize { self . input . current_token_start () } }
};
}

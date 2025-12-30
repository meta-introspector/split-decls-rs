// Generated macro for impl_504 (impl)
macro_rules! Depcrate_stream_tokenimpl_504 {
() => {
// Module: crate::stream::token
// Provides: {"impl_504"}
// Dependencies: {}
impl < T > Location for TokenSlice < '_ , T > where T : Location , { # [inline (always)] fn previous_token_end (& self) -> usize { self . previous_token_end () . or_else (| | self . current_token_start ()) . unwrap_or (0) } # [inline (always)] fn current_token_start (& self) -> usize { self . current_token_start () . or_else (| | self . previous_token_end ()) . unwrap_or (0) } }
};
}

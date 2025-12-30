// Generated macro for impl_453 (impl)
macro_rules! Depcrate_stream_statefulimpl_453 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_453"}
// Dependencies: {}
impl < I , S > Location for Stateful < I , S > where I : Location , { # [inline (always)] fn previous_token_end (& self) -> usize { self . input . previous_token_end () } # [inline (always)] fn current_token_start (& self) -> usize { self . input . current_token_start () } }
};
}

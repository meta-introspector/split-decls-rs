// Generated macro for macro_207 (macro)
macro_rules! Depcrate_hedge_delaymacro_207 {
() => {
// Module: crate::hedge::delay
// Provides: {"macro_207"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] pub struct ResponseFuture < Request , S > where S : Service < Request >, { service : Option < S >, # [pin] state : State < Request , Oneshot < S , Request >>, } }
};
}

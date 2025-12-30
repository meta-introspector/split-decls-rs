// Generated macro for macro_565 (macro)
macro_rules! Depcrate_retry_futuremacro_565 {
() => {
// Module: crate::retry::future
// Provides: {"macro_565"}
// Dependencies: {}
pin_project ! { # [doc = " The [`Future`] returned by a [`Retry`] service."] # [derive (Debug)] pub struct ResponseFuture < P , S , Request > where P : Policy < Request , S :: Response , S :: Error >, S : Service < Request >, { request : Option < Request >, # [pin] retry : Retry < P , S >, # [pin] state : State < S :: Future , P :: Future >, } }
};
}

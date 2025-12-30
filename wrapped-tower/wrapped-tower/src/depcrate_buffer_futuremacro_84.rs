// Generated macro for macro_84 (macro)
macro_rules! Depcrate_buffer_futuremacro_84 {
() => {
// Module: crate::buffer::future
// Provides: {"macro_84"}
// Dependencies: {}
pin_project ! { # [doc = " Future that completes when the buffered service eventually services the submitted request."] # [derive (Debug)] pub struct ResponseFuture < T > { # [pin] state : ResponseState < T >, } }
};
}

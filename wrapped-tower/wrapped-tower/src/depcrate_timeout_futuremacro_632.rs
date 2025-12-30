// Generated macro for macro_632 (macro)
macro_rules! Depcrate_timeout_futuremacro_632 {
() => {
// Module: crate::timeout::future
// Provides: {"macro_632"}
// Dependencies: {}
pin_project ! { # [doc = " [`Timeout`] response future"] # [doc = ""] # [doc = " [`Timeout`]: crate::timeout::Timeout"] # [derive (Debug)] pub struct ResponseFuture < T > { # [pin] response : T , # [pin] sleep : Sleep , } }
};
}

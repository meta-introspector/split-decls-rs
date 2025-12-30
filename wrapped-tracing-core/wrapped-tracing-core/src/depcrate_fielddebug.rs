// Generated macro for debug (function)
macro_rules! Depcrate_fielddebug {
() => {
// Module: crate::field
// Provides: {"debug"}
// Dependencies: {}
# [doc = " Wraps a type implementing `fmt::Debug` as a `Value` that can be"] # [doc = " recorded using its `Debug` implementation."] pub fn debug < T > (t : T) -> DebugValue < T > where T : fmt :: Debug , { DebugValue (t) }
};
}

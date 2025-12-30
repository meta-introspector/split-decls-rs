// Generated macro for opaque (function)
macro_rules! Depcrateopaque {
() => {
// Module: crate
// Provides: {"opaque"}
// Dependencies: {}
pub fn opaque < T : Debug > (value : & T) -> Opaque { Opaque (format ! ("{value:?}")) }
};
}

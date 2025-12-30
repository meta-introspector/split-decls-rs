// Generated macro for create_default_session_globals_then (function)
macro_rules! Depcratecreate_default_session_globals_then {
() => {
// Module: crate
// Provides: {"create_default_session_globals_then"}
// Dependencies: {}
# [doc = " Default edition, no source map."] pub fn create_default_session_globals_then < R > (f : impl FnOnce () -> R) -> R { create_session_globals_then (edition :: DEFAULT_EDITION , & [] , None , f) }
};
}

// Generated macro for set_session_globals_then (function)
macro_rules! Depcrateset_session_globals_then {
() => {
// Module: crate
// Provides: {"set_session_globals_then"}
// Dependencies: {}
pub fn set_session_globals_then < R > (session_globals : & SessionGlobals , f : impl FnOnce () -> R) -> R { assert ! (! SESSION_GLOBALS . is_set () , "SESSION_GLOBALS should never be overwritten! \
         Use another thread if you need another SessionGlobals") ; SESSION_GLOBALS . set (session_globals , f) }
};
}

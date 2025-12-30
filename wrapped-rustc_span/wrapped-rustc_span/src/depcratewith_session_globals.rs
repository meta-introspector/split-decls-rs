// Generated macro for with_session_globals (function)
macro_rules! Depcratewith_session_globals {
() => {
// Module: crate
// Provides: {"with_session_globals"}
// Dependencies: {}
# [inline] pub fn with_session_globals < R , F > (f : F) -> R where F : FnOnce (& SessionGlobals) -> R , { SESSION_GLOBALS . with (f) }
};
}

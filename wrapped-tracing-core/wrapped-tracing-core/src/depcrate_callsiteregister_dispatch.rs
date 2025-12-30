// Generated macro for register_dispatch (function)
macro_rules! Depcrate_callsiteregister_dispatch {
() => {
// Module: crate::callsite
// Provides: {"register_dispatch"}
// Dependencies: {}
pub (crate) fn register_dispatch (dispatch : & Dispatch) { let dispatchers = DISPATCHERS . register_dispatch (dispatch) ; dispatch . subscriber () . on_register_dispatch (dispatch) ; CALLSITES . rebuild_interest (dispatchers) ; }
};
}

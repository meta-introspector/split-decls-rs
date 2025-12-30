// Generated macro for set_panic_hook (function)
macro_rules! Depcrate_utilsset_panic_hook {
() => {
// Module: crate::utils
// Provides: {"set_panic_hook"}
// Dependencies: {}
pub fn set_panic_hook () { # [cfg (feature = "console_error_panic_hook")] console_error_panic_hook :: set_once () ; }
};
}

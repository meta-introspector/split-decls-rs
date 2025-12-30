// Generated macro for restore_default (function)
macro_rules! Depcrate_low_level_signal_detailsrestore_default {
() => {
// Module: crate::low_level::signal_details
// Provides: {"restore_default"}
// Dependencies: {}
# [cfg (windows)] fn restore_default (signal : c_int) -> Result < () , Error > { unsafe { if libc :: signal (signal , 0) == 0 { Ok (()) } else { Err (Error :: last_os_error ()) } } }
};
}

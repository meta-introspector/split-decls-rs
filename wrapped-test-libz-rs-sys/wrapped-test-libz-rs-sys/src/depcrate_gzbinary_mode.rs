// Generated macro for binary_mode (function)
macro_rules! Depcrate_gzbinary_mode {
() => {
// Module: crate::gz
// Provides: {"binary_mode"}
// Dependencies: {}
fn binary_mode (mode : c_int) -> c_int { # [cfg (target_os = "windows")] { mode | libc :: O_BINARY } # [cfg (not (target_os = "windows"))] { mode } }
};
}

// Generated macro for filetime_to_u64 (function)
macro_rules! Depcrate_windows_processfiletime_to_u64 {
() => {
// Module: crate::windows::process
// Provides: {"filetime_to_u64"}
// Dependencies: {}
# [inline (always)] const fn filetime_to_u64 (ft : FILETIME) -> u64 { ((ft . dwHighDateTime as u64) << 32) | (ft . dwLowDateTime as u64) }
};
}

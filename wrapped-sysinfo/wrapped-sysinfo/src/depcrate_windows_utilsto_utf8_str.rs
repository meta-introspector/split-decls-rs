// Generated macro for to_utf8_str (function)
macro_rules! Depcrate_windows_utilsto_utf8_str {
() => {
// Module: crate::windows::utils
// Provides: {"to_utf8_str"}
// Dependencies: {}
# [cfg (any (feature = "user" , feature = "system"))] pub (crate) unsafe fn to_utf8_str (p : windows :: core :: PWSTR) -> String { if p . is_null () { return String :: new () ; } unsafe { p . to_string () . unwrap_or_else (| _e | { sysinfo_debug ! ("Failed to convert to UTF-16 string: {}" , _e) ; String :: new () }) } }
};
}

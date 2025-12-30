// Generated macro for os_string_from_zero_terminated (function)
macro_rules! Depcrate_windows_diskos_string_from_zero_terminated {
() => {
// Module: crate::windows::disk
// Provides: {"os_string_from_zero_terminated"}
// Dependencies: {}
fn os_string_from_zero_terminated (name : & [u16]) -> OsString { let len = name . iter () . position (| & x | x == 0) . unwrap_or (name . len ()) ; OsString :: from_wide (& name [.. len]) }
};
}

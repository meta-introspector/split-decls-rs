// Generated macro for null_terminated_wchar_to_string (function)
macro_rules! Depcrate_windows_processnull_terminated_wchar_to_string {
() => {
// Module: crate::windows::process
// Provides: {"null_terminated_wchar_to_string"}
// Dependencies: {}
unsafe fn null_terminated_wchar_to_string (slice : & [u16]) -> OsString { match slice . iter () . position (| & x | x == 0) { Some (pos) => OsString :: from_wide (& slice [.. pos]) , None => OsString :: from_wide (slice) , } }
};
}

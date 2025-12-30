// Generated macro for strings_to_osstrings (function)
macro_rules! Depcrate_diststrings_to_osstrings {
() => {
// Module: crate::dist
// Provides: {"strings_to_osstrings"}
// Dependencies: {}
pub fn strings_to_osstrings (strings : & [String]) -> Vec < OsString > { strings . iter () . map (| arg | std :: ffi :: OsStr :: new (arg) . to_os_string ()) . collect :: < Vec < _ > > () }
};
}

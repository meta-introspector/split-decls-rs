// Generated macro for sysinfo_system_version (function)
macro_rules! Depcrate_c_interfacesysinfo_system_version {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_system_version"}
// Dependencies: {}
# [doc = " Equivalent of [`System::version()`][crate::System#method.version]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_system_version () -> RString { if let Some (c) = System :: os_version () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

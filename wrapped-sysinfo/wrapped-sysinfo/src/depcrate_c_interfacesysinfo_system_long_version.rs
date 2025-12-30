// Generated macro for sysinfo_system_long_version (function)
macro_rules! Depcrate_c_interfacesysinfo_system_long_version {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_system_long_version"}
// Dependencies: {}
# [doc = " Equivalent of [`System::long_os_version()`][crate::System#method.long_os_version]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_system_long_version () -> RString { if let Some (c) = System :: long_os_version () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

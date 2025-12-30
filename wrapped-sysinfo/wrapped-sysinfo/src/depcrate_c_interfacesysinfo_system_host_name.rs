// Generated macro for sysinfo_system_host_name (function)
macro_rules! Depcrate_c_interfacesysinfo_system_host_name {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_system_host_name"}
// Dependencies: {}
# [doc = " Equivalent of [`System::host_name()`][crate::System#method.host_name]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_system_host_name () -> RString { if let Some (c) = System :: host_name () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

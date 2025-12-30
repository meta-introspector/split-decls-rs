// Generated macro for sysinfo_system_kernel_version (function)
macro_rules! Depcrate_c_interfacesysinfo_system_kernel_version {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_system_kernel_version"}
// Dependencies: {}
# [doc = " Equivalent of [`System::kernel_version()`][crate::System#method.kernel_version]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_system_kernel_version () -> RString { if let Some (c) = System :: kernel_version () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

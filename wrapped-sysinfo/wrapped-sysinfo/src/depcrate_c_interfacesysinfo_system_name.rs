// Generated macro for sysinfo_system_name (function)
macro_rules! Depcrate_c_interfacesysinfo_system_name {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_system_name"}
// Dependencies: {}
# [doc = " Equivalent of [`System::name()`][crate::System#method.name]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_system_name () -> RString { if let Some (c) = System :: name () . and_then (| p | CString :: new (p) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

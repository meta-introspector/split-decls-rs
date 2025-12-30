// Generated macro for sysinfo_motherboard_version (function)
macro_rules! Depcrate_c_interfacesysinfo_motherboard_version {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_motherboard_version"}
// Dependencies: {}
# [doc = " Equivalent of [`Motherboard::version()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_motherboard_version () -> RString { if let Some (c) = Motherboard :: new () . and_then (| m | m . version ()) . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

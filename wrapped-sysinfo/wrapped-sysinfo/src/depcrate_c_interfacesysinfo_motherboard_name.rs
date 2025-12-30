// Generated macro for sysinfo_motherboard_name (function)
macro_rules! Depcrate_c_interfacesysinfo_motherboard_name {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_motherboard_name"}
// Dependencies: {}
# [doc = " Equivalent of [`Motherboard::name()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_motherboard_name () -> RString { if let Some (c) = Motherboard :: new () . and_then (| m | m . name ()) . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

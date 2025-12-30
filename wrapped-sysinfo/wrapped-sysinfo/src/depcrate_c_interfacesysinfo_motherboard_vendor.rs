// Generated macro for sysinfo_motherboard_vendor (function)
macro_rules! Depcrate_c_interfacesysinfo_motherboard_vendor {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_motherboard_vendor"}
// Dependencies: {}
# [doc = " Equivalent of [`Motherboard::vendor_name()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_motherboard_vendor () -> RString { if let Some (c) = Motherboard :: new () . and_then (| m | m . vendor_name ()) . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

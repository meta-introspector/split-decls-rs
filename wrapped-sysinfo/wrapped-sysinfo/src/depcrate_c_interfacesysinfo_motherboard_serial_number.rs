// Generated macro for sysinfo_motherboard_serial_number (function)
macro_rules! Depcrate_c_interfacesysinfo_motherboard_serial_number {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_motherboard_serial_number"}
// Dependencies: {}
# [doc = " Equivalent of [`Motherboard::serial_number()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_motherboard_serial_number () -> RString { if let Some (c) = Motherboard :: new () . and_then (| m | m . serial_number ()) . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

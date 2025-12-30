// Generated macro for sysinfo_product_version (function)
macro_rules! Depcrate_c_interfacesysinfo_product_version {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_product_version"}
// Dependencies: {}
# [doc = " Equivalent of [`Product::version()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_product_version () -> RString { if let Some (c) = Product :: version () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

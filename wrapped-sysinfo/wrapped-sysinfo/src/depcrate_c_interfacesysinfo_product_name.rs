// Generated macro for sysinfo_product_name (function)
macro_rules! Depcrate_c_interfacesysinfo_product_name {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_product_name"}
// Dependencies: {}
# [doc = " Equivalent of [`Product::name()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_product_name () -> RString { if let Some (c) = Product :: name () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

// Generated macro for sysinfo_product_vendor_name (function)
macro_rules! Depcrate_c_interfacesysinfo_product_vendor_name {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_product_vendor_name"}
// Dependencies: {}
# [doc = " Equivalent of [`Product::vendor_name()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_product_vendor_name () -> RString { if let Some (c) = Product :: vendor_name () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

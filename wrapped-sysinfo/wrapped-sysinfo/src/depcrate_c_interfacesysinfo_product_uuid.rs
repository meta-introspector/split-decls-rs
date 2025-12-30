// Generated macro for sysinfo_product_uuid (function)
macro_rules! Depcrate_c_interfacesysinfo_product_uuid {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_product_uuid"}
// Dependencies: {}
# [doc = " Equivalent of [`Product::uuid()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_product_uuid () -> RString { if let Some (c) = Product :: uuid () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

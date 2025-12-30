// Generated macro for sysinfo_product_family (function)
macro_rules! Depcrate_c_interfacesysinfo_product_family {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_product_family"}
// Dependencies: {}
# [doc = " Equivalent of [`Product::family()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_product_family () -> RString { if let Some (c) = Product :: family () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

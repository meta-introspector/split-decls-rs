// Generated macro for sysinfo_product_stock_keeping_unit (function)
macro_rules! Depcrate_c_interfacesysinfo_product_stock_keeping_unit {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_product_stock_keeping_unit"}
// Dependencies: {}
# [doc = " Equivalent of [`Product::stock_keeping_unit()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_product_stock_keeping_unit () -> RString { if let Some (c) = Product :: stock_keeping_unit () . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}

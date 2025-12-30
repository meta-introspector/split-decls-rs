// Generated macro for __externref_drop_slice (function)
macro_rules! Depcrate_externref__externref_drop_slice {
() => {
// Module: crate::externref
// Provides: {"__externref_drop_slice"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn __externref_drop_slice (ptr : * mut JsValue , len : usize) { for slot in slice :: from_raw_parts_mut (ptr , len) { __externref_table_dealloc (slot . idx as usize) ; } }
};
}

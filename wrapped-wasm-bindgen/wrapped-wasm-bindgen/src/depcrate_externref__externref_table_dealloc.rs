// Generated macro for __externref_table_dealloc (function)
macro_rules! Depcrate_externref__externref_table_dealloc {
() => {
// Module: crate::externref
// Provides: {"__externref_table_dealloc"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn __externref_table_dealloc (idx : usize) { if idx < __rt :: JSIDX_RESERVED as usize { return ; } unsafe { __wbindgen_externref_table_set_null (idx) ; } HEAP_SLAB . 0 . borrow_mut () . dealloc (idx) }
};
}

// Generated macro for __externref_table_alloc (function)
macro_rules! Depcrate_externref__externref_table_alloc {
() => {
// Module: crate::externref
// Provides: {"__externref_table_alloc"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn __externref_table_alloc () -> usize { HEAP_SLAB . 0 . borrow_mut () . alloc () }
};
}

// Generated macro for impl_516 (impl)
macro_rules! Depcrate_tableimpl_516 {
() => {
// Module: crate::table
// Provides: {"impl_516"}
// Dependencies: {}
impl Drop for Page { fn drop (& mut self) { let len = * self . allocated . get_mut () ; unsafe { (self . slot_vtable . drop_impl) (self . data . as_ptr () , len , & self . memo_types) } ; } }
};
}

// Generated macro for impl_203 (impl)
macro_rules! Depcrate_alpn_listimpl_203 {
() => {
// Module: crate::alpn_list
// Provides: {"impl_203"}
// Dependencies: {}
impl Drop for AlpnList { fn drop (& mut self) { unsafe { alloc :: dealloc (self . memory . as_ptr () , self . layout) ; } } }
};
}

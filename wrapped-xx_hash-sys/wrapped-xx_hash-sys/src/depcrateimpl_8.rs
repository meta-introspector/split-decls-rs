// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Drop for XxHash32 { fn drop (& mut self) { let retval = unsafe { XXH32_freeState (self . 0) } ; assert_eq ! (retval , XXH_OK) ; } }
};
}

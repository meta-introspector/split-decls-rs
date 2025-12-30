// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Drop for XxHash64 { fn drop (& mut self) { let retval = unsafe { XXH64_freeState (self . 0) } ; assert_eq ! (retval , XXH_OK) ; } }
};
}

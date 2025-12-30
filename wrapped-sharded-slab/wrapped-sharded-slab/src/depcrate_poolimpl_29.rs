// Generated macro for impl_29 (impl)
macro_rules! Depcrate_poolimpl_29 {
() => {
// Module: crate::pool
// Provides: {"impl_29"}
// Dependencies: {}
impl < T , C > Drop for RefMut < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { fn drop (& mut self) { test_println ! (" -> drop RefMut: try clearing data") ; let should_clear = unsafe { self . inner . release () } ; if should_clear { self . shard . clear_after_release (self . key) ; } } }
};
}

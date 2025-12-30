// Generated macro for impl_223 (impl)
macro_rules! Depcrateimpl_223 {
() => {
// Module: crate
// Provides: {"impl_223"}
// Dependencies: {}
impl < T , C : cfg :: Config > Drop for Entry < '_ , T , C > { fn drop (& mut self) { let should_remove = unsafe { self . inner . release () } ; if should_remove { self . shard . clear_after_release (self . key) } } }
};
}

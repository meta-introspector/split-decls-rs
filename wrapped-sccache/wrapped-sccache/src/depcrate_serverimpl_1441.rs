// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_serverimpl_1441 {
() => {
// Module: crate::server
// Provides: {"impl_1441"}
// Dependencies: {}
impl Drop for Info { fn drop (& mut self) { if let Some (waker) = self . waker . as_ref () { waker . wake_by_ref () ; } } }
};
}

// Generated macro for impl_70 (impl)
macro_rules! Depcrate_threadimpl_70 {
() => {
// Module: crate::thread
// Provides: {"impl_70"}
// Dependencies: {}
impl < T > Drop for JoinHandle < T > { fn drop (& mut self) { if ! self . allow_leak { return ; } if let Some (join_handle) = self . inner . take () { join_handle . detach () ; } } }
};
}

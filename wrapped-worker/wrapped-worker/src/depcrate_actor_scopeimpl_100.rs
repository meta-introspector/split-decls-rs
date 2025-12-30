// Generated macro for impl_100 (impl)
macro_rules! Depcrate_actor_scopeimpl_100 {
() => {
// Module: crate::actor::scope
// Provides: {"impl_100"}
// Dependencies: {}
impl < W : Worker > Clone for WorkerScope < W > { fn clone (& self) -> Self { WorkerScope { state : self . state . clone () , post_msg : self . post_msg . clone () , } } }
};
}

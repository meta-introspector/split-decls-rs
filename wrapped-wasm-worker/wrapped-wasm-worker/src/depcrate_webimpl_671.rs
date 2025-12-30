// Generated macro for impl_671 (impl)
macro_rules! Depcrate_webimpl_671 {
() => {
// Module: crate::web
// Provides: {"impl_671"}
// Dependencies: {}
impl < T > Future for ScopedJoinHandleFuture < '_ , '_ , T > { type Output = crate :: Result < T > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ScopedJoinHandle :: poll (self . 0 , cx) } }
};
}

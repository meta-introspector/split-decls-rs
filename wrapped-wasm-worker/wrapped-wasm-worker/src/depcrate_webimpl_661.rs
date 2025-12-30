// Generated macro for impl_661 (impl)
macro_rules! Depcrate_webimpl_661 {
() => {
// Module: crate::web
// Provides: {"impl_661"}
// Dependencies: {}
impl < T > Future for JoinHandleFuture < '_ , T > { type Output = crate :: Result < T > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { JoinHandle :: poll (self . 0 , cx) } }
};
}

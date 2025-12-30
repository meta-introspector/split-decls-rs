// Generated macro for impl_678 (impl)
macro_rules! Depcrate_webimpl_678 {
() => {
// Module: crate::web
// Provides: {"impl_678"}
// Dependencies: {}
impl < T > Future for ScopeJoinFuture < '_ , '_ , T > { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}

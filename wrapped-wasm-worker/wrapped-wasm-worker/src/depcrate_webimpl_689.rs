// Generated macro for impl_689 (impl)
macro_rules! Depcrate_webimpl_689 {
() => {
// Module: crate::web
// Provides: {"impl_689"}
// Dependencies: {}
impl Future for YieldNowFuture { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}

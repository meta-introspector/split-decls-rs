// Generated macro for impl_25 (impl)
macro_rules! Depcrate_futureimpl_25 {
() => {
// Module: crate::future
// Provides: {"impl_25"}
// Dependencies: {}
impl Future for TimeoutFuture { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { Future :: poll (Pin :: new (& mut self . rx) , cx) . map (| t | t . unwrap_throw ()) } }
};
}

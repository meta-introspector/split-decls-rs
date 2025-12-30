// Generated macro for impl_511 (impl)
macro_rules! Depcrate_reconnect_futureimpl_511 {
() => {
// Module: crate::reconnect::future
// Provides: {"impl_511"}
// Dependencies: {}
impl < F , E > ResponseFuture < F , E > { pub (crate) fn new (inner : F) -> Self { ResponseFuture { inner : Inner :: future (inner) , } } pub (crate) fn error (error : E) -> Self { ResponseFuture { inner : Inner :: error (Some (error)) , } } }
};
}

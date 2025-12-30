// Generated macro for impl_708 (impl)
macro_rules! Depcrate_limit_futureimpl_708 {
() => {
// Module: crate::limit::future
// Provides: {"impl_708"}
// Dependencies: {}
impl < F > ResponseFuture < F > { pub (crate) fn payload_too_large () -> Self { Self { inner : ResponseFutureInner :: PayloadTooLarge , } } pub (crate) fn new (future : F) -> Self { Self { inner : ResponseFutureInner :: Future { future } , } } }
};
}

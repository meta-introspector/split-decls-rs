// Generated macro for impl_273 (impl)
macro_rules! Depcrate_limit_concurrency_futureimpl_273 {
() => {
// Module: crate::limit::concurrency::future
// Provides: {"impl_273"}
// Dependencies: {}
impl < T > ResponseFuture < T > { pub (crate) fn new (inner : T , _permit : OwnedSemaphorePermit) -> ResponseFuture < T > { ResponseFuture { inner , _permit } } }
};
}

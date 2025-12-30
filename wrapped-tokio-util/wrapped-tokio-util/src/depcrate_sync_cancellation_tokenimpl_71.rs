// Generated macro for impl_71 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_71 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a , F : Future > RunUntilCancelledFuture < 'a , F > { pub (crate) fn new (cancellation_token : & 'a CancellationToken , future : F) -> Self { Self { cancellation : cancellation_token . cancelled () , future , } } }
};
}

// Generated macro for impl_75 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_75 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_75"}
// Dependencies: {}
impl < F : Future > RunUntilCancelledFutureOwned < F > { pub (crate) fn new (cancellation_token : CancellationToken , future : F) -> Self { Self { cancellation : cancellation_token . cancelled_owned () , future , } } }
};
}

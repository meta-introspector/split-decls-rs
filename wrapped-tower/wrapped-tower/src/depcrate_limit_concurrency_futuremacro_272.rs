// Generated macro for macro_272 (macro)
macro_rules! Depcrate_limit_concurrency_futuremacro_272 {
() => {
// Module: crate::limit::concurrency::future
// Provides: {"macro_272"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`ConcurrencyLimit`] service."] # [doc = ""] # [doc = " [`ConcurrencyLimit`]: crate::limit::ConcurrencyLimit"] # [derive (Debug)] pub struct ResponseFuture < T > { # [pin] inner : T , _permit : OwnedSemaphorePermit , } }
};
}

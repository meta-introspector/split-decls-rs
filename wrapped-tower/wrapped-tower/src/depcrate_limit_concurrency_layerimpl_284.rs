// Generated macro for impl_284 (impl)
macro_rules! Depcrate_limit_concurrency_layerimpl_284 {
() => {
// Module: crate::limit::concurrency::layer
// Provides: {"impl_284"}
// Dependencies: {}
impl GlobalConcurrencyLimitLayer { # [doc = " Create a new `GlobalConcurrencyLimitLayer`."] pub fn new (max : usize) -> Self { Self :: with_semaphore (Arc :: new (Semaphore :: new (max))) } # [doc = " Create a new `GlobalConcurrencyLimitLayer` from a `Arc<Semaphore>`"] pub fn with_semaphore (semaphore : Arc < Semaphore >) -> Self { GlobalConcurrencyLimitLayer { semaphore } } }
};
}

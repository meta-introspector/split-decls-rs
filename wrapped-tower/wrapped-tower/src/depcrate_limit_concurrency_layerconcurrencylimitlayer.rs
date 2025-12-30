// Generated macro for ConcurrencyLimitLayer (struct)
macro_rules! Depcrate_limit_concurrency_layerConcurrencyLimitLayer {
() => {
// Module: crate::limit::concurrency::layer
// Provides: {"ConcurrencyLimitLayer"}
// Dependencies: {}
# [doc = " Enforces a limit on the concurrent number of requests the underlying"] # [doc = " service can handle."] # [derive (Debug , Clone)] pub struct ConcurrencyLimitLayer { max : usize , }
};
}

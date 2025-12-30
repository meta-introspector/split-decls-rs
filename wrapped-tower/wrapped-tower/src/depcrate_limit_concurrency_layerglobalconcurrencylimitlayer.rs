// Generated macro for GlobalConcurrencyLimitLayer (struct)
macro_rules! Depcrate_limit_concurrency_layerGlobalConcurrencyLimitLayer {
() => {
// Module: crate::limit::concurrency::layer
// Provides: {"GlobalConcurrencyLimitLayer"}
// Dependencies: {}
# [doc = " Enforces a limit on the concurrent number of requests the underlying"] # [doc = " service can handle."] # [doc = ""] # [doc = " Unlike [`ConcurrencyLimitLayer`], which enforces a per-service concurrency"] # [doc = " limit, this layer accepts a owned semaphore (`Arc<Semaphore>`) which can be"] # [doc = " shared across multiple services."] # [doc = ""] # [doc = " Cloning this layer will not create a new semaphore."] # [derive (Debug , Clone)] pub struct GlobalConcurrencyLimitLayer { semaphore : Arc < Semaphore > , }
};
}

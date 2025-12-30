// Generated macro for impl_285 (impl)
macro_rules! Depcrate_limit_concurrency_layerimpl_285 {
() => {
// Module: crate::limit::concurrency::layer
// Provides: {"impl_285"}
// Dependencies: {}
impl < S > Layer < S > for GlobalConcurrencyLimitLayer { type Service = ConcurrencyLimit < S > ; fn layer (& self , service : S) -> Self :: Service { ConcurrencyLimit :: with_semaphore (service , self . semaphore . clone ()) } }
};
}

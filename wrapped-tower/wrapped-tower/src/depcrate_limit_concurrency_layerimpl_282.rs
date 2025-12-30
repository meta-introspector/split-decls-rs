// Generated macro for impl_282 (impl)
macro_rules! Depcrate_limit_concurrency_layerimpl_282 {
() => {
// Module: crate::limit::concurrency::layer
// Provides: {"impl_282"}
// Dependencies: {}
impl < S > Layer < S > for ConcurrencyLimitLayer { type Service = ConcurrencyLimit < S > ; fn layer (& self , service : S) -> Self :: Service { ConcurrencyLimit :: new (service , self . max) } }
};
}

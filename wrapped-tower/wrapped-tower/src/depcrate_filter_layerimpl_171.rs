// Generated macro for impl_171 (impl)
macro_rules! Depcrate_filter_layerimpl_171 {
() => {
// Module: crate::filter::layer
// Provides: {"impl_171"}
// Dependencies: {}
impl < U : Clone , S > Layer < S > for AsyncFilterLayer < U > { type Service = AsyncFilter < S , U > ; fn layer (& self , service : S) -> Self :: Service { let predicate = self . predicate . clone () ; AsyncFilter :: new (service , predicate) } }
};
}

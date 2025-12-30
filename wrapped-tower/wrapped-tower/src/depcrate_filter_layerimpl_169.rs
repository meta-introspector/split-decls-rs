// Generated macro for impl_169 (impl)
macro_rules! Depcrate_filter_layerimpl_169 {
() => {
// Module: crate::filter::layer
// Provides: {"impl_169"}
// Dependencies: {}
impl < U : Clone , S > Layer < S > for FilterLayer < U > { type Service = Filter < S , U > ; fn layer (& self , service : S) -> Self :: Service { let predicate = self . predicate . clone () ; Filter :: new (service , predicate) } }
};
}

// Generated macro for impl_716 (impl)
macro_rules! Depcrate_limit_layerimpl_716 {
() => {
// Module: crate::limit::layer
// Provides: {"impl_716"}
// Dependencies: {}
impl < S > Layer < S > for RequestBodyLimitLayer { type Service = RequestBodyLimit < S > ; fn layer (& self , inner : S) -> Self :: Service { RequestBodyLimit { inner , limit : self . limit , } } }
};
}

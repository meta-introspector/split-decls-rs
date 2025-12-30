// Generated macro for impl_972 (impl)
macro_rules! Depcrate_builderimpl_972 {
() => {
// Module: crate::builder
// Provides: {"impl_972"}
// Dependencies: {}
impl < S , L > Layer < S > for ServiceBuilder < L > where L : Layer < S > , { type Service = L :: Service ; fn layer (& self , inner : S) -> Self :: Service { self . layer . layer (inner) } }
};
}

// Generated macro for impl_1006 (impl)
macro_rules! Depcrate_normalize_pathimpl_1006 {
() => {
// Module: crate::normalize_path
// Provides: {"impl_1006"}
// Dependencies: {}
impl < S > Layer < S > for NormalizePathLayer { type Service = NormalizePath < S > ; fn layer (& self , inner : S) -> Self :: Service { NormalizePath { mode : self . mode , inner , } } }
};
}

// Generated macro for impl_7 (impl)
macro_rules! Depcrate_identityimpl_7 {
() => {
// Module: crate::identity
// Provides: {"impl_7"}
// Dependencies: {}
impl < S > Layer < S > for Identity { type Service = S ; fn layer (& self , inner : S) -> Self :: Service { inner } }
};
}

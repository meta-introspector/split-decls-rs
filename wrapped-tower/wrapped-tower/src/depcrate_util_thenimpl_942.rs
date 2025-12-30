// Generated macro for impl_942 (impl)
macro_rules! Depcrate_util_thenimpl_942 {
() => {
// Module: crate::util::then
// Provides: {"impl_942"}
// Dependencies: {}
impl < S , F > Layer < S > for ThenLayer < F > where F : Clone , { type Service = Then < S , F > ; fn layer (& self , inner : S) -> Self :: Service { Then { f : self . f . clone () , inner , } } }
};
}

// Generated macro for impl_670 (impl)
macro_rules! Depcrate_util_and_thenimpl_670 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_670"}
// Dependencies: {}
impl < S , F > Layer < S > for AndThenLayer < F > where F : Clone , { type Service = AndThen < S , F > ; fn layer (& self , inner : S) -> Self :: Service { AndThen { f : self . f . clone () , inner , } } }
};
}

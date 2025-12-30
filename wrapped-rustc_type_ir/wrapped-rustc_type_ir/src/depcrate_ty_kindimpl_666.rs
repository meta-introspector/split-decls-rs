// Generated macro for impl_666 (impl)
macro_rules! Depcrate_ty_kindimpl_666 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_666"}
// Dependencies: {}
impl < I : Interner > FnSig < I > { pub fn inputs (self) -> I :: FnInputTys { self . inputs_and_output . inputs () } pub fn output (self) -> I :: Ty { self . inputs_and_output . output () } pub fn is_fn_trait_compatible (self) -> bool { let FnSig { safety , abi , c_variadic , .. } = self ; ! c_variadic && safety . is_safe () && abi . is_rust () } }
};
}

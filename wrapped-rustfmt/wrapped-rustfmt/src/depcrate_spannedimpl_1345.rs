// Generated macro for impl_1345 (impl)
macro_rules! Depcrate_spannedimpl_1345 {
() => {
// Module: crate::spanned
// Provides: {"impl_1345"}
// Dependencies: {}
impl Spanned for ast :: FnRetTy { fn span (& self) -> Span { match * self { ast :: FnRetTy :: Default (span) => span , ast :: FnRetTy :: Ty (ref ty) => ty . span , } } }
};
}

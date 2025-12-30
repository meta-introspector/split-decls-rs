// Generated macro for impl_1323 (impl)
macro_rules! Depcrate_spannedimpl_1323 {
() => {
// Module: crate::spanned
// Provides: {"impl_1323"}
// Dependencies: {}
impl Spanned for ast :: FnRetTy { fn span (& self) -> Span { match * self { ast :: FnRetTy :: Default (span) => span , ast :: FnRetTy :: Ty (ref ty) => ty . span , } } }
};
}

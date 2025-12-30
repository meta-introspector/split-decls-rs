// Generated macro for impl_1324 (impl)
macro_rules! Depcrate_spannedimpl_1324 {
() => {
// Module: crate::spanned
// Provides: {"impl_1324"}
// Dependencies: {}
impl Spanned for ast :: GenericArg { fn span (& self) -> Span { match * self { ast :: GenericArg :: Lifetime (ref lt) => lt . ident . span , ast :: GenericArg :: Type (ref ty) => ty . span () , ast :: GenericArg :: Const (ref _const) => _const . value . span () , } } }
};
}

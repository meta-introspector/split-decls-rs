// Generated macro for impl_1350 (impl)
macro_rules! Depcrate_spannedimpl_1350 {
() => {
// Module: crate::spanned
// Provides: {"impl_1350"}
// Dependencies: {}
impl Spanned for ast :: PreciseCapturingArg { fn span (& self) -> Span { match self { ast :: PreciseCapturingArg :: Lifetime (lt) => lt . ident . span , ast :: PreciseCapturingArg :: Arg (path , _) => path . span , } } }
};
}

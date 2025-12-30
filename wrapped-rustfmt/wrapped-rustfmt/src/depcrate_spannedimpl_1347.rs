// Generated macro for impl_1347 (impl)
macro_rules! Depcrate_spannedimpl_1347 {
() => {
// Module: crate::spanned
// Provides: {"impl_1347"}
// Dependencies: {}
impl Spanned for ast :: GenericBound { fn span (& self) -> Span { match * self { ast :: GenericBound :: Trait (ref ptr) => ptr . span , ast :: GenericBound :: Outlives (ref l) => l . ident . span , ast :: GenericBound :: Use (_ , span) => span , } } }
};
}

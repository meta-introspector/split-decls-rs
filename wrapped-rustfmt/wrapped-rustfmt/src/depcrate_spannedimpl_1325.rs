// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_spannedimpl_1325 {
() => {
// Module: crate::spanned
// Provides: {"impl_1325"}
// Dependencies: {}
impl Spanned for ast :: GenericBound { fn span (& self) -> Span { match * self { ast :: GenericBound :: Trait (ref ptr) => ptr . span , ast :: GenericBound :: Outlives (ref l) => l . ident . span , ast :: GenericBound :: Use (_ , span) => span , } } }
};
}

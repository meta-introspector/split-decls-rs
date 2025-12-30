// Generated macro for impl_1509 (impl)
macro_rules! Depcrate_typesimpl_1509 {
() => {
// Module: crate::types
// Provides: {"impl_1509"}
// Dependencies: {}
impl < 'a > Spanned for SegmentParam < 'a > { fn span (& self) -> Span { match * self { SegmentParam :: Const (const_) => const_ . value . span , SegmentParam :: LifeTime (lt) => lt . ident . span , SegmentParam :: Type (ty) => ty . span , SegmentParam :: Binding (binding) => binding . span , } } }
};
}

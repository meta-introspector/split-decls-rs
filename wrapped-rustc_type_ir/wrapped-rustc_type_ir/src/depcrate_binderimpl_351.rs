// Generated macro for impl_351 (impl)
macro_rules! Depcrate_binderimpl_351 {
() => {
// Module: crate::binder
// Provides: {"impl_351"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator , A > ExactSizeIterator for IterInstantiated < I , Iter , A > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { }
};
}

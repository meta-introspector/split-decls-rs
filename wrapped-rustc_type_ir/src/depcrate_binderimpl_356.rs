// Generated macro for impl_356 (impl)
macro_rules! Depcrate_binderimpl_356 {
() => {
// Module: crate::binder
// Provides: {"impl_356"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator > ExactSizeIterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { }
};
}

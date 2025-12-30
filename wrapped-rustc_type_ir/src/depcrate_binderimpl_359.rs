// Generated macro for impl_359 (impl)
macro_rules! Depcrate_binderimpl_359 {
() => {
// Module: crate::binder
// Provides: {"impl_359"}
// Dependencies: {}
impl < Iter : IntoIterator > DoubleEndedIterator for IterIdentityCopied < Iter > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| i | * i) } }
};
}

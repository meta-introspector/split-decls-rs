// Generated macro for impl_355 (impl)
macro_rules! Depcrate_binderimpl_355 {
() => {
// Module: crate::binder
// Provides: {"impl_355"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator > DoubleEndedIterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| value | { EarlyBinder { value : * value , _tcx : PhantomData } . instantiate (self . cx , self . args) }) } }
};
}

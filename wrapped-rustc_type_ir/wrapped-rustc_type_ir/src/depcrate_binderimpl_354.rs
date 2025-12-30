// Generated macro for impl_354 (impl)
macro_rules! Depcrate_binderimpl_354 {
() => {
// Module: crate::binder
// Provides: {"impl_354"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator > Iterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { type Item = < Iter :: Item as Deref > :: Target ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| value | { EarlyBinder { value : * value , _tcx : PhantomData } . instantiate (self . cx , self . args) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}

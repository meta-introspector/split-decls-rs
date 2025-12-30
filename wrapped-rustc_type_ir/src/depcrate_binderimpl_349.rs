// Generated macro for impl_349 (impl)
macro_rules! Depcrate_binderimpl_349 {
() => {
// Module: crate::binder
// Provides: {"impl_349"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator , A > Iterator for IterInstantiated < I , Iter , A > where Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { type Item = Iter :: Item ; fn next (& mut self) -> Option < Self :: Item > { Some (EarlyBinder { value : self . it . next () ? , _tcx : PhantomData } . instantiate (self . cx , self . args) ,) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}

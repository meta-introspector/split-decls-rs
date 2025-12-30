// Generated macro for impl_350 (impl)
macro_rules! Depcrate_binderimpl_350 {
() => {
// Module: crate::binder
// Provides: {"impl_350"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator , A > DoubleEndedIterator for IterInstantiated < I , Iter , A > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { fn next_back (& mut self) -> Option < Self :: Item > { Some (EarlyBinder { value : self . it . next_back () ? , _tcx : PhantomData } . instantiate (self . cx , self . args) ,) } }
};
}

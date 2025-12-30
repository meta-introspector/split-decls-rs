// Generated macro for impl_402 (impl)
macro_rules! Depcrate_revisionimpl_402 {
() => {
// Module: crate::revision
// Provides: {"impl_402"}
// Dependencies: {}
impl AtomicRevision { pub (crate) const fn start () -> Self { Self { data : AtomicUsize :: new (START) , } } pub (crate) fn load (& self) -> Revision { Revision { generation : unsafe { NonZeroUsize :: new_unchecked (self . data . load (Ordering :: Acquire)) } , } } pub (crate) fn store (& self , r : Revision) { self . data . store (r . as_usize () , Ordering :: Release) ; } }
};
}

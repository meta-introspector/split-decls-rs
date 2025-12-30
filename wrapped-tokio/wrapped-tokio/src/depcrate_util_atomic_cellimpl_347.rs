// Generated macro for impl_347 (impl)
macro_rules! Depcrate_util_atomic_cellimpl_347 {
() => {
// Module: crate::util::atomic_cell
// Provides: {"impl_347"}
// Dependencies: {}
impl < T > AtomicCell < T > { pub (crate) fn new (data : Option < Box < T > >) -> AtomicCell < T > { AtomicCell { data : AtomicPtr :: new (to_raw (data)) , } } pub (crate) fn swap (& self , val : Option < Box < T > >) -> Option < Box < T > > { let old = self . data . swap (to_raw (val) , AcqRel) ; from_raw (old) } pub (crate) fn set (& self , val : Box < T >) { let _ = self . swap (Some (val)) ; } pub (crate) fn take (& self) -> Option < Box < T > > { self . swap (None) } }
};
}

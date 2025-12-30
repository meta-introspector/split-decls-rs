// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > DoubleEndedIterator for IterMut < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { while let Some ((key , entry)) = self . entries . next_back () { if let Entry :: Occupied (ref mut v) = * entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } }
};
}

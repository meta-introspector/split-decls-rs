// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < T > DoubleEndedIterator for Iter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { while let Some ((key , entry)) = self . entries . next_back () { if let Entry :: Occupied (ref v) = * entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } }
};
}

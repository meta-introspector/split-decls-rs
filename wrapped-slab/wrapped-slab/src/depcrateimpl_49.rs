// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < T > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < Self :: Item > { while let Some ((key , entry)) = self . entries . next_back () { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } }
};
}

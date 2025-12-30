// Generated macro for impl_100 (impl)
macro_rules! Depcrate_cycleimpl_100 {
() => {
// Module: crate::cycle
// Provides: {"impl_100"}
// Dependencies: {}
impl DoubleEndedIterator for CycleHeadsIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { loop { let next = self . inner . next_back () ? ; if next . removed . load (Ordering :: Relaxed) { continue ; } return Some (next) ; } } }
};
}

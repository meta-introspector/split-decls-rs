// Generated macro for impl_98 (impl)
macro_rules! Depcrate_cycleimpl_98 {
() => {
// Module: crate::cycle
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a > Iterator for CycleHeadsIterator < 'a > { type Item = & 'a CycleHead ; fn next (& mut self) -> Option < Self :: Item > { loop { let next = self . inner . next () ? ; if next . removed . load (Ordering :: Relaxed) { continue ; } return Some (next) ; } } }
};
}

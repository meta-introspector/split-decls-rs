// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > Iterator for Drain < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { for entry in & mut self . inner { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some (v) ; } } debug_assert_eq ! (self . len , 0) ; None } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}

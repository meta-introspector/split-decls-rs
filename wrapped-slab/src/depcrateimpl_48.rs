// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = (usize , T) ; fn next (& mut self) -> Option < Self :: Item > { for (key , entry) in & mut self . entries { if let Entry :: Occupied (v) = entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}

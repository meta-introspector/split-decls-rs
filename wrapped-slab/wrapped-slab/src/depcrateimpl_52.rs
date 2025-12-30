// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = (usize , & 'a T) ; fn next (& mut self) -> Option < Self :: Item > { for (key , entry) in & mut self . entries { if let Entry :: Occupied (ref v) = * entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}

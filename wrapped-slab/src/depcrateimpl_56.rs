// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = (usize , & 'a mut T) ; fn next (& mut self) -> Option < Self :: Item > { for (key , entry) in & mut self . entries { if let Entry :: Occupied (ref mut v) = * entry { self . len -= 1 ; return Some ((key , v)) ; } } debug_assert_eq ! (self . len , 0) ; None } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}

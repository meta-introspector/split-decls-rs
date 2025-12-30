// Generated macro for impl_17 (impl)
macro_rules! Depcrate_inline_array_iterimpl_17 {
() => {
// Module: crate::inline_array::iter
// Provides: {"impl_17"}
// Dependencies: {}
impl < A , T > Iterator for Iter < A , T > { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . array . remove (0) } fn size_hint (& self) -> (usize , Option < usize >) { (self . array . len () , Some (self . array . len ())) } }
};
}

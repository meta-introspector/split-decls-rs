// Generated macro for impl_22 (impl)
macro_rules! Depcrate_inline_array_iterimpl_22 {
() => {
// Module: crate::inline_array::iter
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , A , T > Iterator for Drain < 'a , A , T > { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . array . remove (0) } fn size_hint (& self) -> (usize , Option < usize >) { (self . array . len () , Some (self . array . len ())) } }
};
}

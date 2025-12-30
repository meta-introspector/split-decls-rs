// Generated macro for impl_915 (impl)
macro_rules! Depcrate_punctuatedimpl_915 {
() => {
// Module: crate::punctuated
// Provides: {"impl_915"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}

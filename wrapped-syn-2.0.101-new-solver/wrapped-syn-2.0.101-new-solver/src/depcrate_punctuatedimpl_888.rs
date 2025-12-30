// Generated macro for impl_888 (impl)
macro_rules! Depcrate_punctuatedimpl_888 {
() => {
// Module: crate::punctuated
// Provides: {"impl_888"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}

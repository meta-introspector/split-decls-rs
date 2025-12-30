// Generated macro for impl_911 (impl)
macro_rules! Depcrate_punctuatedimpl_911 {
() => {
// Module: crate::punctuated
// Provides: {"impl_911"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}

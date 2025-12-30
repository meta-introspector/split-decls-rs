// Generated macro for impl_421 (impl)
macro_rules! Depcrate_genericsimpl_421 {
() => {
// Module: crate::generics
// Provides: {"impl_421"}
// Dependencies: {}
impl < 'a > Iterator for Lifetimes < 'a > { type Item = & 'a LifetimeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Lifetime (lifetime) = self . 0 . next () ? { Some (lifetime) } else { self . next () } } }
};
}

// Generated macro for impl_429 (impl)
macro_rules! Depcrate_genericsimpl_429 {
() => {
// Module: crate::generics
// Provides: {"impl_429"}
// Dependencies: {}
impl < 'a > Iterator for ConstParams < 'a > { type Item = & 'a ConstParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Const (const_param) = self . 0 . next () ? { Some (const_param) } else { self . next () } } }
};
}

// Generated macro for impl_425 (impl)
macro_rules! Depcrate_genericsimpl_425 {
() => {
// Module: crate::generics
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'a > Iterator for TypeParams < 'a > { type Item = & 'a TypeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Type (type_param) = self . 0 . next () ? { Some (type_param) } else { self . next () } } }
};
}

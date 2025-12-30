// Generated macro for impl_443 (impl)
macro_rules! Depcrate_genericsimpl_443 {
() => {
// Module: crate::generics
// Provides: {"impl_443"}
// Dependencies: {}
impl < 'a > Iterator for ConstParamsMut < 'a > { type Item = & 'a mut ConstParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Const (const_param) = self . 0 . next () ? { Some (const_param) } else { self . next () } } }
};
}

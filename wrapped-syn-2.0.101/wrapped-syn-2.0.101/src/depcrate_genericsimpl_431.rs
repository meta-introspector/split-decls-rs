// Generated macro for impl_431 (impl)
macro_rules! Depcrate_genericsimpl_431 {
() => {
// Module: crate::generics
// Provides: {"impl_431"}
// Dependencies: {}
impl < 'a > Iterator for ConstParamsMut < 'a > { type Item = & 'a mut ConstParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Const (const_param) = self . 0 . next () ? { Some (const_param) } else { self . next () } } }
};
}

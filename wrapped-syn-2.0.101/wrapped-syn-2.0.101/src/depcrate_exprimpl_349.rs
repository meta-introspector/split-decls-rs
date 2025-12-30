// Generated macro for impl_349 (impl)
macro_rules! Depcrate_exprimpl_349 {
() => {
// Module: crate::expr
// Provides: {"impl_349"}
// Dependencies: {}
impl From < usize > for Index { fn from (index : usize) -> Index { assert ! (index < u32 :: MAX as usize) ; Index { index : index as u32 , span : Span :: call_site () , } } }
};
}

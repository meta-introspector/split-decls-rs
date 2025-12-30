// Generated macro for impl_172 (impl)
macro_rules! Depcrate_relateimpl_172 {
() => {
// Module: crate::relate
// Provides: {"impl_172"}
// Dependencies: {}
impl < I : Interner > VarianceDiagInfo < I > { # [doc = " Mirrors `Variance::xform` - used to 'combine' the existing"] # [doc = " and new `VarianceDiagInfo`s when our variance changes."] pub fn xform (self , other : VarianceDiagInfo < I >) -> VarianceDiagInfo < I > { match self { VarianceDiagInfo :: None => other , VarianceDiagInfo :: Invariant { .. } => self , } } }
};
}

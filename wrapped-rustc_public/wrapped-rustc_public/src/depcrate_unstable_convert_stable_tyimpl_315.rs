// Generated macro for impl_315 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_315 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_315"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: PredicatePolarity { type T = crate :: ty :: PredicatePolarity ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: PredicatePolarity :: * ; match self { Positive => crate :: ty :: PredicatePolarity :: Positive , Negative => crate :: ty :: PredicatePolarity :: Negative , } } }
};
}

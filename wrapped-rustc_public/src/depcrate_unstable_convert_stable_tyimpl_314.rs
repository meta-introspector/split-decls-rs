// Generated macro for impl_314 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_314 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ImplPolarity { type T = crate :: ty :: ImplPolarity ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: ImplPolarity :: * ; match self { Positive => crate :: ty :: ImplPolarity :: Positive , Negative => crate :: ty :: ImplPolarity :: Negative , Reservation => crate :: ty :: ImplPolarity :: Reservation , } } }
};
}

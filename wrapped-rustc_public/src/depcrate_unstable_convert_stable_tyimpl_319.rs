// Generated macro for impl_319 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_319 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_319"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: Variance { type T = crate :: mir :: Variance ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Bivariant => crate :: mir :: Variance :: Bivariant , ty :: Contravariant => crate :: mir :: Variance :: Contravariant , ty :: Covariant => crate :: mir :: Variance :: Covariant , ty :: Invariant => crate :: mir :: Variance :: Invariant , } } }
};
}

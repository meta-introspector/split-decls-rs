// Generated macro for impl_328 (impl)
macro_rules! Depcrate_unstable_convert_stableimpl_328 {
() => {
// Module: crate::unstable::convert::stable
// Provides: {"impl_328"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_hir :: Safety { type T = crate :: mir :: Safety ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_hir :: Safety :: Unsafe => crate :: mir :: Safety :: Unsafe , rustc_hir :: Safety :: Safe => crate :: mir :: Safety :: Safe , } } }
};
}

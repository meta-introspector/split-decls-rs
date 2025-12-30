// Generated macro for impl_330 (impl)
macro_rules! Depcrate_unstable_convert_stableimpl_330 {
() => {
// Module: crate::unstable::convert::stable
// Provides: {"impl_330"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_hir :: CoroutineSource { type T = crate :: mir :: CoroutineSource ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_hir :: CoroutineSource ; match self { CoroutineSource :: Block => crate :: mir :: CoroutineSource :: Block , CoroutineSource :: Closure => crate :: mir :: CoroutineSource :: Closure , CoroutineSource :: Fn => crate :: mir :: CoroutineSource :: Fn , } } }
};
}

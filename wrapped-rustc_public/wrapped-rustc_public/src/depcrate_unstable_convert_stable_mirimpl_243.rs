// Generated macro for impl_243 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_243 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_243"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: RetagKind { type T = crate :: mir :: RetagKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: RetagKind ; match self { RetagKind :: FnEntry => crate :: mir :: RetagKind :: FnEntry , RetagKind :: TwoPhase => crate :: mir :: RetagKind :: TwoPhase , RetagKind :: Raw => crate :: mir :: RetagKind :: Raw , RetagKind :: Default => crate :: mir :: RetagKind :: Default , } } }
};
}

// Generated macro for impl_244 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_244 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_244"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: UnwindAction { type T = crate :: mir :: UnwindAction ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: UnwindAction ; match self { UnwindAction :: Continue => crate :: mir :: UnwindAction :: Continue , UnwindAction :: Unreachable => crate :: mir :: UnwindAction :: Unreachable , UnwindAction :: Terminate (_) => crate :: mir :: UnwindAction :: Terminate , UnwindAction :: Cleanup (bb) => crate :: mir :: UnwindAction :: Cleanup (bb . as_usize ()) , } } }
};
}

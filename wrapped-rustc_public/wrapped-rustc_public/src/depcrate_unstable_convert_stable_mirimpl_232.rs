// Generated macro for impl_232 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_232 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: MutBorrowKind { type T = crate :: mir :: MutBorrowKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: MutBorrowKind :: * ; match * self { Default => crate :: mir :: MutBorrowKind :: Default , TwoPhaseBorrow => crate :: mir :: MutBorrowKind :: TwoPhaseBorrow , ClosureCapture => crate :: mir :: MutBorrowKind :: ClosureCapture , } } }
};
}

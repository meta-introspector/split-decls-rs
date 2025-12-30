// Generated macro for impl_233 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_233 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_233"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: FakeBorrowKind { type T = crate :: mir :: FakeBorrowKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: FakeBorrowKind :: * ; match * self { Deep => crate :: mir :: FakeBorrowKind :: Deep , Shallow => crate :: mir :: FakeBorrowKind :: Shallow , } } }
};
}

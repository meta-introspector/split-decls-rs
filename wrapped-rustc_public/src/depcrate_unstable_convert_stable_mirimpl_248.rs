// Generated macro for impl_248 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_248 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_248"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: UnOp { type T = crate :: mir :: UnOp ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: UnOp ; match self { UnOp :: Not => crate :: mir :: UnOp :: Not , UnOp :: Neg => crate :: mir :: UnOp :: Neg , UnOp :: PtrMetadata => crate :: mir :: UnOp :: PtrMetadata , } } }
};
}

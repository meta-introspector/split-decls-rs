// Generated macro for impl_229 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_229 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: Mutability { type T = crate :: mir :: Mutability ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_hir :: Mutability :: * ; match * self { Not => crate :: mir :: Mutability :: Not , Mut => crate :: mir :: Mutability :: Mut , } } }
};
}

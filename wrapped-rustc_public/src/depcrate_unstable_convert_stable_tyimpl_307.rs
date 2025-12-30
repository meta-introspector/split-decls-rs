// Generated macro for impl_307 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_307 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_307"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ClosureKind { type T = crate :: ty :: ClosureKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: ClosureKind :: * ; match self { Fn => crate :: ty :: ClosureKind :: Fn , FnMut => crate :: ty :: ClosureKind :: FnMut , FnOnce => crate :: ty :: ClosureKind :: FnOnce , } } }
};
}

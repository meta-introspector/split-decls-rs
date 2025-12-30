// Generated macro for impl_236 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_236 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_236"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: FakeReadCause { type T = crate :: mir :: FakeReadCause ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: FakeReadCause :: * ; match self { ForMatchGuard => crate :: mir :: FakeReadCause :: ForMatchGuard , ForMatchedPlace (local_def_id) => { crate :: mir :: FakeReadCause :: ForMatchedPlace (opaque (local_def_id)) } ForGuardBinding => crate :: mir :: FakeReadCause :: ForGuardBinding , ForLet (local_def_id) => crate :: mir :: FakeReadCause :: ForLet (opaque (local_def_id)) , ForIndex => crate :: mir :: FakeReadCause :: ForIndex , } } }
};
}

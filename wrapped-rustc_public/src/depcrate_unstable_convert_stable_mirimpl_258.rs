// Generated macro for impl_258 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_258 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: interpret :: ErrorHandled { type T = Error ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { bridge :: Error :: new (format ! ("{self:?}")) } }
};
}

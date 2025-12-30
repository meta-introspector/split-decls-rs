// Generated macro for impl_242 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_242 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_242"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: Local { type T = crate :: mir :: Local ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . as_usize () } }
};
}

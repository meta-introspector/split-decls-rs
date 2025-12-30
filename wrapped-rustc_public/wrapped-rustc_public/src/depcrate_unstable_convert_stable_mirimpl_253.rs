// Generated macro for impl_253 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_253 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: interpret :: ConstAllocation < 'tcx > { type T = Allocation ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . inner () . stable (tables , cx) } }
};
}

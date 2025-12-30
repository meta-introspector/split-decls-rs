// Generated macro for impl_254 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_254 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: interpret :: Allocation { type T = crate :: ty :: Allocation ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_public_bridge :: context :: AllocRangeHelpers ; alloc :: allocation_filter (self , cx . alloc_range (rustc_abi :: Size :: ZERO , self . size ()) , tables , cx ,) } }
};
}

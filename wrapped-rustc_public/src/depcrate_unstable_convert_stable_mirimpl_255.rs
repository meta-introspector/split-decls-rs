// Generated macro for impl_255 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_255 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: interpret :: AllocId { type T = crate :: mir :: alloc :: AllocId ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , _ : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . create_alloc_id (* self) } }
};
}

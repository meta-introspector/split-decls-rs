// Generated macro for impl_251 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_251 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: Terminator < 'tcx > { type T = crate :: mir :: Terminator ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: mir :: Terminator ; Terminator { kind : self . kind . stable (tables , cx) , span : self . source_info . span . stable (tables , cx) , } } }
};
}

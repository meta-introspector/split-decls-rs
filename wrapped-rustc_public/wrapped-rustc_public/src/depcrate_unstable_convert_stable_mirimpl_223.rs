// Generated macro for impl_223 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_223 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: Statement < 'tcx > { type T = crate :: mir :: Statement ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { Statement { kind : self . kind . stable (tables , cx) , span : self . source_info . span . stable (tables , cx) , } } }
};
}

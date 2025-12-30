// Generated macro for impl_224 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_224 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: SourceInfo { type T = crate :: mir :: SourceInfo ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: SourceInfo { span : self . span . stable (tables , cx) , scope : self . scope . into () } } }
};
}

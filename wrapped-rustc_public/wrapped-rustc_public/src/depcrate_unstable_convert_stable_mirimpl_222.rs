// Generated macro for impl_222 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_222 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: VarDebugInfo < 'tcx > { type T = crate :: mir :: VarDebugInfo ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: VarDebugInfo { name : self . name . to_string () , source_info : self . source_info . stable (tables , cx) , composite : self . composite . as_ref () . map (| composite | composite . stable (tables , cx)) , value : self . value . stable (tables , cx) , argument_index : self . argument_index , } } }
};
}

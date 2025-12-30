// Generated macro for impl_225 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_225 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: VarDebugInfoFragment < 'tcx > { type T = crate :: mir :: VarDebugInfoFragment ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { VarDebugInfoFragment { ty : self . ty . stable (tables , cx) , projection : self . projection . iter () . map (| e | e . stable (tables , cx)) . collect () , } } }
};
}

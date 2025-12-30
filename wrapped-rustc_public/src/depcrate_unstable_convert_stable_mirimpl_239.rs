// Generated macro for impl_239 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_239 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: Place < 'tcx > { type T = crate :: mir :: Place ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: Place { local : self . local . as_usize () , projection : self . projection . iter () . map (| e | e . stable (tables , cx)) . collect () , } } }
};
}

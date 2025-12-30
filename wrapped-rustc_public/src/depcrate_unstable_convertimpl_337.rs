// Generated macro for impl_337 (impl)
macro_rules! Depcrate_unstable_convertimpl_337 {
() => {
// Module: crate::unstable::convert
// Provides: {"impl_337"}
// Dependencies: {}
impl < 'tcx , T > Stable < 'tcx > for & [T] where T : Stable < 'tcx > , { type T = Vec < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . iter () . map (| e | e . stable (tables , cx)) . collect () } }
};
}

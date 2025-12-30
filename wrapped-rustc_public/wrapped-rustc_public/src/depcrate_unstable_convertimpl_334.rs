// Generated macro for impl_334 (impl)
macro_rules! Depcrate_unstable_convertimpl_334 {
() => {
// Module: crate::unstable::convert
// Provides: {"impl_334"}
// Dependencies: {}
impl < 'tcx , T > Stable < 'tcx > for & T where T : Stable < 'tcx > , { type T = T :: T ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (* self) . stable (tables , cx) } }
};
}

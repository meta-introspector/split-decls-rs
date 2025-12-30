// Generated macro for impl_335 (impl)
macro_rules! Depcrate_unstable_convertimpl_335 {
() => {
// Module: crate::unstable::convert
// Provides: {"impl_335"}
// Dependencies: {}
impl < 'tcx , T > Stable < 'tcx > for Option < T > where T : Stable < 'tcx > , { type T = Option < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . as_ref () . map (| value | value . stable (tables , cx)) } }
};
}

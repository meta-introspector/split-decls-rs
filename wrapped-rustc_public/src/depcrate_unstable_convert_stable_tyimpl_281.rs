// Generated macro for impl_281 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_281 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: GenericArgs < 'tcx > { type T = crate :: ty :: GenericArgs ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { GenericArgs (self . iter () . map (| arg | arg . kind () . stable (tables , cx)) . collect ()) } }
};
}

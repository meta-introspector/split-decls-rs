// Generated macro for impl_166 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_166 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_166"}
// Dependencies: {}
impl < T > RustcInternal for Option < T > where T : RustcInternal , { type T < 'tcx > = Option < T :: T < 'tcx > > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . as_ref () . map (| inner | inner . internal (tables , tcx)) } }
};
}

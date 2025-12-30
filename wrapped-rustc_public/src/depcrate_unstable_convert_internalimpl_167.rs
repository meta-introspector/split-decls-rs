// Generated macro for impl_167 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_167 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_167"}
// Dependencies: {}
impl < T > RustcInternal for Vec < T > where T : RustcInternal , { type T < 'tcx > = Vec < T :: T < 'tcx > > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . iter () . map (| e | e . internal (tables , tcx)) . collect () } }
};
}

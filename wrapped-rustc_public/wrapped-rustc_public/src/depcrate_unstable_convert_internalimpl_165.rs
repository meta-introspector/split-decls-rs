// Generated macro for impl_165 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_165 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_165"}
// Dependencies: {}
impl < T > RustcInternal for & T where T : RustcInternal , { type T < 'tcx > = T :: T < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { (* self) . internal (tables , tcx) } }
};
}

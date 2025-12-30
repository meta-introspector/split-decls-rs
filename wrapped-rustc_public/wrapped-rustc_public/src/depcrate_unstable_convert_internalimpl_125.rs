// Generated macro for impl_125 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_125 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_125"}
// Dependencies: {}
impl RustcInternal for DefId { type T < 'tcx > = rustc_span :: def_id :: DefId ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . def_ids [* self]) . unwrap () } }
};
}

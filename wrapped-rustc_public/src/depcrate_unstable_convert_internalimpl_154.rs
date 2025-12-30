// Generated macro for impl_154 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_154 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_154"}
// Dependencies: {}
impl RustcInternal for AllocId { type T < 'tcx > = rustc_middle :: mir :: interpret :: AllocId ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . alloc_ids [* self]) . unwrap () } }
};
}

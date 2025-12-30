// Generated macro for impl_160 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_160 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_160"}
// Dependencies: {}
impl RustcInternal for Layout { type T < 'tcx > = rustc_abi :: Layout < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . layouts [* self]) . unwrap () } }
};
}

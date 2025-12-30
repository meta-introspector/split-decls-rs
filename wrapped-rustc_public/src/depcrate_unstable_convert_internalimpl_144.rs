// Generated macro for impl_144 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_144 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_144"}
// Dependencies: {}
impl RustcInternal for Instance { type T < 'tcx > = rustc_ty :: Instance < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . instances [self . def]) . unwrap () } }
};
}

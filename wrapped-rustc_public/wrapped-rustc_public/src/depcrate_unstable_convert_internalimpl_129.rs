// Generated macro for impl_129 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_129 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_129"}
// Dependencies: {}
impl RustcInternal for Ty { type T < 'tcx > = InternalTy < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . types [* self]) . unwrap () } }
};
}

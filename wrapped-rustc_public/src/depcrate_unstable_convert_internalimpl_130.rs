// Generated macro for impl_130 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_130 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_130"}
// Dependencies: {}
impl RustcInternal for TyConst { type T < 'tcx > = InternalConst < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . ty_consts [self . id]) . unwrap () } }
};
}

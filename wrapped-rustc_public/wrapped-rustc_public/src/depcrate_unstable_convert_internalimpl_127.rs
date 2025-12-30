// Generated macro for impl_127 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_127 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_127"}
// Dependencies: {}
impl RustcInternal for GenericArgKind { type T < 'tcx > = rustc_ty :: GenericArg < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { let arg : rustc_ty :: GenericArg < 'tcx > = match self { GenericArgKind :: Lifetime (reg) => reg . internal (tables , tcx) . into () , GenericArgKind :: Type (ty) => ty . internal (tables , tcx) . into () , GenericArgKind :: Const (cnst) => cnst . internal (tables , tcx) . into () , } ; tcx . lift (arg) . unwrap () } }
};
}

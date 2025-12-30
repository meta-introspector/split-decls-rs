// Generated macro for impl_151 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_151 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_151"}
// Dependencies: {}
impl RustcInternal for TermKind { type T < 'tcx > = rustc_ty :: Term < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { TermKind :: Type (ty) => ty . internal (tables , tcx) . into () , TermKind :: Const (cnst) => cnst . internal (tables , tcx) . into () , } } }
};
}

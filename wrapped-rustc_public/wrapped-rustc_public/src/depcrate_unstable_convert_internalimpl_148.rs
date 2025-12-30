// Generated macro for impl_148 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_148 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_148"}
// Dependencies: {}
impl RustcInternal for DynKind { type T < 'tcx > = rustc_ty :: DynKind ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { DynKind :: Dyn => rustc_ty :: DynKind :: Dyn , } } }
};
}

// Generated macro for impl_155 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_155 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_155"}
// Dependencies: {}
impl RustcInternal for ClosureKind { type T < 'tcx > = rustc_ty :: ClosureKind ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { ClosureKind :: Fn => rustc_ty :: ClosureKind :: Fn , ClosureKind :: FnMut => rustc_ty :: ClosureKind :: FnMut , ClosureKind :: FnOnce => rustc_ty :: ClosureKind :: FnOnce , } } }
};
}

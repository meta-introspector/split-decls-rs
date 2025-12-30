// Generated macro for impl_164 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_164 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_164"}
// Dependencies: {}
impl RustcInternal for UnOp { type T < 'tcx > = rustc_middle :: mir :: UnOp ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { UnOp :: Not => rustc_middle :: mir :: UnOp :: Not , UnOp :: Neg => rustc_middle :: mir :: UnOp :: Neg , UnOp :: PtrMetadata => rustc_middle :: mir :: UnOp :: PtrMetadata , } } }
};
}

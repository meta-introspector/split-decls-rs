// Generated macro for impl_136 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_136 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_136"}
// Dependencies: {}
impl RustcInternal for Mutability { type T < 'tcx > = rustc_ty :: Mutability ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { Mutability :: Not => rustc_ty :: Mutability :: Not , Mutability :: Mut => rustc_ty :: Mutability :: Mut , } } }
};
}

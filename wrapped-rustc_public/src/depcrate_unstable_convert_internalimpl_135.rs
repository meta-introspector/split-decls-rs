// Generated macro for impl_135 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_135 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_135"}
// Dependencies: {}
impl RustcInternal for FloatTy { type T < 'tcx > = rustc_ty :: FloatTy ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { FloatTy :: F16 => rustc_ty :: FloatTy :: F16 , FloatTy :: F32 => rustc_ty :: FloatTy :: F32 , FloatTy :: F64 => rustc_ty :: FloatTy :: F64 , FloatTy :: F128 => rustc_ty :: FloatTy :: F128 , } } }
};
}

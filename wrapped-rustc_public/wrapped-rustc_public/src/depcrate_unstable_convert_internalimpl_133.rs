// Generated macro for impl_133 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_133 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_133"}
// Dependencies: {}
impl RustcInternal for IntTy { type T < 'tcx > = rustc_ty :: IntTy ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { IntTy :: Isize => rustc_ty :: IntTy :: Isize , IntTy :: I8 => rustc_ty :: IntTy :: I8 , IntTy :: I16 => rustc_ty :: IntTy :: I16 , IntTy :: I32 => rustc_ty :: IntTy :: I32 , IntTy :: I64 => rustc_ty :: IntTy :: I64 , IntTy :: I128 => rustc_ty :: IntTy :: I128 , } } }
};
}

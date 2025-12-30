// Generated macro for impl_134 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_134 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_134"}
// Dependencies: {}
impl RustcInternal for UintTy { type T < 'tcx > = rustc_ty :: UintTy ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { UintTy :: Usize => rustc_ty :: UintTy :: Usize , UintTy :: U8 => rustc_ty :: UintTy :: U8 , UintTy :: U16 => rustc_ty :: UintTy :: U16 , UintTy :: U32 => rustc_ty :: UintTy :: U32 , UintTy :: U64 => rustc_ty :: UintTy :: U64 , UintTy :: U128 => rustc_ty :: UintTy :: U128 , } } }
};
}

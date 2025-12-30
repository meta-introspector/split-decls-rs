// Generated macro for impl_289 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_289 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: IntTy { type T = IntTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: IntTy :: Isize => IntTy :: Isize , ty :: IntTy :: I8 => IntTy :: I8 , ty :: IntTy :: I16 => IntTy :: I16 , ty :: IntTy :: I32 => IntTy :: I32 , ty :: IntTy :: I64 => IntTy :: I64 , ty :: IntTy :: I128 => IntTy :: I128 , } } }
};
}

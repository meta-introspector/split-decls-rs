// Generated macro for impl_291 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_291 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_291"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: FloatTy { type T = FloatTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: FloatTy :: F16 => FloatTy :: F16 , ty :: FloatTy :: F32 => FloatTy :: F32 , ty :: FloatTy :: F64 => FloatTy :: F64 , ty :: FloatTy :: F128 => FloatTy :: F128 , } } }
};
}

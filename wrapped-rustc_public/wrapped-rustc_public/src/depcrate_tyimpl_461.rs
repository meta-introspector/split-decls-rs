// Generated macro for impl_461 (impl)
macro_rules! Depcrate_tyimpl_461 {
() => {
// Module: crate::ty
// Provides: {"impl_461"}
// Dependencies: {}
impl TyConst { pub fn new (kind : TyConstKind , id : TyConstId) -> TyConst { Self { kind , id } } # [doc = " Retrieve the constant kind."] pub fn kind (& self) -> & TyConstKind { & self . kind } # [doc = " Creates an interned usize constant."] pub fn try_from_target_usize (val : u64) -> Result < Self , Error > { with (| cx | cx . try_new_ty_const_uint (val . into () , UintTy :: Usize)) } # [doc = " Try to evaluate to a target `usize`."] pub fn eval_target_usize (& self) -> Result < u64 , Error > { with (| cx | cx . eval_target_usize_ty (self)) } }
};
}

// Generated macro for impl_465 (impl)
macro_rules! Depcrate_tyimpl_465 {
() => {
// Module: crate::ty
// Provides: {"impl_465"}
// Dependencies: {}
impl MirConst { # [doc = " Build a constant. Note that this should only be used by the compiler."] pub fn new (kind : ConstantKind , ty : Ty , id : MirConstId) -> MirConst { MirConst { kind , ty , id } } # [doc = " Retrieve the constant kind."] pub fn kind (& self) -> & ConstantKind { & self . kind } # [doc = " Get the constant type."] pub fn ty (& self) -> Ty { self . ty } # [doc = " Try to evaluate to a target `usize`."] pub fn eval_target_usize (& self) -> Result < u64 , Error > { with (| cx | cx . eval_target_usize (self)) } # [doc = " Create a constant that represents a new zero-sized constant of type T."] # [doc = " Fails if the type is not a ZST or if it doesn't have a known size."] pub fn try_new_zero_sized (ty : Ty) -> Result < MirConst , Error > { with (| cx | cx . try_new_const_zst (ty)) } # [doc = " Build a new constant that represents the given string."] # [doc = ""] # [doc = " Note that there is no guarantee today about duplication of the same constant."] # [doc = " I.e.: Calling this function multiple times with the same argument may or may not return"] # [doc = " the same allocation."] pub fn from_str (value : & str) -> MirConst { with (| cx | cx . new_const_str (value)) } # [doc = " Build a new constant that represents the given boolean value."] pub fn from_bool (value : bool) -> MirConst { with (| cx | cx . new_const_bool (value)) } # [doc = " Build a new constant that represents the given unsigned integer."] pub fn try_from_uint (value : u128 , uint_ty : UintTy) -> Result < MirConst , Error > { with (| cx | cx . try_new_const_uint (value , uint_ty)) } }
};
}

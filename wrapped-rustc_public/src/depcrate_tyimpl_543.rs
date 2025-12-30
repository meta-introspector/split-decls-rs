// Generated macro for impl_543 (impl)
macro_rules! Depcrate_tyimpl_543 {
() => {
// Module: crate::ty
// Provides: {"impl_543"}
// Dependencies: {}
impl PolyFnSig { # [doc = " Compute a `FnAbi` suitable for indirect calls, i.e. to `fn` pointers."] # [doc = ""] # [doc = " NB: this doesn't handle virtual calls - those should use `Instance::fn_abi`"] # [doc = " instead, where the instance is an `InstanceKind::Virtual`."] pub fn fn_ptr_abi (self) -> Result < FnAbi , Error > { with (| cx | cx . fn_ptr_abi (self)) } }
};
}

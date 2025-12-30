// Generated macro for impl_503 (impl)
macro_rules! Depcrate_tyimpl_503 {
() => {
// Module: crate::ty
// Provides: {"impl_503"}
// Dependencies: {}
impl IntrinsicDef { # [doc = " Returns the plain name of the intrinsic."] # [doc = " e.g., `transmute` for `core::intrinsics::transmute`."] pub fn fn_name (& self) -> Symbol { with (| cx | cx . intrinsic_name (* self)) } # [doc = " Returns whether the intrinsic has no meaningful body and all backends"] # [doc = " need to shim all calls to it."] pub fn must_be_overridden (& self) -> bool { with (| cx | ! cx . has_body (self . 0)) } }
};
}

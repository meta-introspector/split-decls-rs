// Generated macro for impl_126 (impl)
macro_rules! Depcrate_error_different_variantimpl_126 {
() => {
// Module: crate::error::different_variant
// Provides: {"impl_126"}
// Dependencies: {}
impl TryFrom < crate :: Error > for DifferentVariant { type Error = Self ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: DifferentVariant (err) => Ok (err) , _ => Err (Self) , } } }
};
}

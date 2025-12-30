// Generated macro for impl_165 (impl)
macro_rules! Depcrate_error_invalid_variantimpl_165 {
() => {
// Module: crate::error::invalid_variant
// Provides: {"impl_165"}
// Dependencies: {}
impl TryFrom < crate :: Error > for InvalidVariant { type Error = crate :: error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: InvalidVariant (err) => Ok (err) , _ => Err (crate :: error :: DifferentVariant) , } } }
};
}

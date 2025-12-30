// Generated macro for impl_149 (impl)
macro_rules! Depcrate_error_indeterminate_offsetimpl_149 {
() => {
// Module: crate::error::indeterminate_offset
// Provides: {"impl_149"}
// Dependencies: {}
impl TryFrom < crate :: Error > for IndeterminateOffset { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: IndeterminateOffset (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

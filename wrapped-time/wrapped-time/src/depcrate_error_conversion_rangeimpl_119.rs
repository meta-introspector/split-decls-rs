// Generated macro for impl_119 (impl)
macro_rules! Depcrate_error_conversion_rangeimpl_119 {
() => {
// Module: crate::error::conversion_range
// Provides: {"impl_119"}
// Dependencies: {}
impl TryFrom < crate :: Error > for ConversionRange { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: ConversionRange (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

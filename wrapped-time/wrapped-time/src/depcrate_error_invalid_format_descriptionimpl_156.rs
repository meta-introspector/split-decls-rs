// Generated macro for impl_156 (impl)
macro_rules! Depcrate_error_invalid_format_descriptionimpl_156 {
() => {
// Module: crate::error::invalid_format_description
// Provides: {"impl_156"}
// Dependencies: {}
impl TryFrom < crate :: Error > for InvalidFormatDescription { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: InvalidFormatDescription (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

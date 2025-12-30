// Generated macro for impl_186 (impl)
macro_rules! Depcrate_error_parse_from_descriptionimpl_186 {
() => {
// Module: crate::error::parse_from_description
// Provides: {"impl_186"}
// Dependencies: {}
impl TryFrom < crate :: Error > for ParseFromDescription { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: ParseFromDescription (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

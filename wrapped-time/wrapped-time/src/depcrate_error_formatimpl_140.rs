// Generated macro for impl_140 (impl)
macro_rules! Depcrate_error_formatimpl_140 {
() => {
// Module: crate::error::format
// Provides: {"impl_140"}
// Dependencies: {}
impl TryFrom < crate :: Error > for Format { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: Format (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

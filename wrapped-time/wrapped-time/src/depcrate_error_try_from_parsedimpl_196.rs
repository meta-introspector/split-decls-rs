// Generated macro for impl_196 (impl)
macro_rules! Depcrate_error_try_from_parsedimpl_196 {
() => {
// Module: crate::error::try_from_parsed
// Provides: {"impl_196"}
// Dependencies: {}
impl TryFrom < crate :: Error > for TryFromParsed { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: TryFromParsed (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

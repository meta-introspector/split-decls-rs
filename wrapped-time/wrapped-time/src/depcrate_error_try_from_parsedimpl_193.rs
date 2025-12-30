// Generated macro for impl_193 (impl)
macro_rules! Depcrate_error_try_from_parsedimpl_193 {
() => {
// Module: crate::error::try_from_parsed
// Provides: {"impl_193"}
// Dependencies: {}
impl TryFrom < TryFromParsed > for error :: ComponentRange { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : TryFromParsed) -> Result < Self , Self :: Error > { match err { TryFromParsed :: ComponentRange (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

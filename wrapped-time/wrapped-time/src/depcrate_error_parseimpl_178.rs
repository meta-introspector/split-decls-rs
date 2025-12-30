// Generated macro for impl_178 (impl)
macro_rules! Depcrate_error_parseimpl_178 {
() => {
// Module: crate::error::parse
// Provides: {"impl_178"}
// Dependencies: {}
impl TryFrom < crate :: Error > for Parse { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: ParseFromDescription (err) => Ok (Self :: ParseFromDescription (err)) , # [allow (deprecated)] crate :: Error :: UnexpectedTrailingCharacters { never } => match never { } , crate :: Error :: TryFromParsed (err) => Ok (Self :: TryFromParsed (err)) , _ => Err (error :: DifferentVariant) , } } }
};
}

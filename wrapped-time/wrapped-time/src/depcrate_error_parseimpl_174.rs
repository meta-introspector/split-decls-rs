// Generated macro for impl_174 (impl)
macro_rules! Depcrate_error_parseimpl_174 {
() => {
// Module: crate::error::parse
// Provides: {"impl_174"}
// Dependencies: {}
impl TryFrom < Parse > for TryFromParsed { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : Parse) -> Result < Self , Self :: Error > { match err { Parse :: TryFromParsed (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

// Generated macro for impl_176 (impl)
macro_rules! Depcrate_error_parseimpl_176 {
() => {
// Module: crate::error::parse
// Provides: {"impl_176"}
// Dependencies: {}
impl TryFrom < Parse > for ParseFromDescription { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : Parse) -> Result < Self , Self :: Error > { match err { Parse :: ParseFromDescription (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

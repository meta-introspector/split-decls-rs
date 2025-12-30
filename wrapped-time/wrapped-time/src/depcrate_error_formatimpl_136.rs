// Generated macro for impl_136 (impl)
macro_rules! Depcrate_error_formatimpl_136 {
() => {
// Module: crate::error::format
// Provides: {"impl_136"}
// Dependencies: {}
impl TryFrom < Format > for error :: ComponentRange { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : Format) -> Result < Self , Self :: Error > { match err { Format :: ComponentRange (err) => Ok (* err) , _ => Err (error :: DifferentVariant) , } } }
};
}

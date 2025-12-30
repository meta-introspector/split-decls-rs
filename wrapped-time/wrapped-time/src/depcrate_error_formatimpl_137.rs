// Generated macro for impl_137 (impl)
macro_rules! Depcrate_error_formatimpl_137 {
() => {
// Module: crate::error::format
// Provides: {"impl_137"}
// Dependencies: {}
impl TryFrom < Format > for io :: Error { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : Format) -> Result < Self , Self :: Error > { match err { Format :: StdIo (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

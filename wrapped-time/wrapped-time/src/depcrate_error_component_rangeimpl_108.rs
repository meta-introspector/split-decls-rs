// Generated macro for impl_108 (impl)
macro_rules! Depcrate_error_component_rangeimpl_108 {
() => {
// Module: crate::error::component_range
// Provides: {"impl_108"}
// Dependencies: {}
impl TryFrom < crate :: Error > for ComponentRange { type Error = error :: DifferentVariant ; # [inline] fn try_from (err : crate :: Error) -> Result < Self , Self :: Error > { match err { crate :: Error :: ComponentRange (err) => Ok (err) , _ => Err (error :: DifferentVariant) , } } }
};
}

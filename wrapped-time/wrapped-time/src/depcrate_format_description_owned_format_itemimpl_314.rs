// Generated macro for impl_314 (impl)
macro_rules! Depcrate_format_description_owned_format_itemimpl_314 {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"impl_314"}
// Dependencies: {}
impl TryFrom < OwnedFormatItem > for Component { type Error = error :: DifferentVariant ; # [inline] fn try_from (value : OwnedFormatItem) -> Result < Self , Self :: Error > { match value { OwnedFormatItem :: Component (component) => Ok (component) , _ => Err (error :: DifferentVariant) , } } }
};
}

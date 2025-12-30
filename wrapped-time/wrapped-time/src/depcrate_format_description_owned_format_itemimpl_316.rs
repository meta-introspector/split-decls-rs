// Generated macro for impl_316 (impl)
macro_rules! Depcrate_format_description_owned_format_itemimpl_316 {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"impl_316"}
// Dependencies: {}
impl TryFrom < OwnedFormatItem > for Vec < OwnedFormatItem > { type Error = error :: DifferentVariant ; # [inline] fn try_from (value : OwnedFormatItem) -> Result < Self , Self :: Error > { match value { OwnedFormatItem :: Compound (items) => Ok (items . into_vec ()) , _ => Err (error :: DifferentVariant) , } } }
};
}

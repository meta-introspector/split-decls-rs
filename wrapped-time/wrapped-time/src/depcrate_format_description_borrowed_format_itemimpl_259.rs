// Generated macro for impl_259 (impl)
macro_rules! Depcrate_format_description_borrowed_format_itemimpl_259 {
() => {
// Module: crate::format_description::borrowed_format_item
// Provides: {"impl_259"}
// Dependencies: {}
impl TryFrom < BorrowedFormatItem < '_ > > for Component { type Error = error :: DifferentVariant ; # [inline] fn try_from (value : BorrowedFormatItem < '_ >) -> Result < Self , Self :: Error > { match value { BorrowedFormatItem :: Component (component) => Ok (component) , _ => Err (error :: DifferentVariant) , } } }
};
}

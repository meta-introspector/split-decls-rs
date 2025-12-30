// Generated macro for impl_261 (impl)
macro_rules! Depcrate_format_description_borrowed_format_itemimpl_261 {
() => {
// Module: crate::format_description::borrowed_format_item
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'a > TryFrom < BorrowedFormatItem < 'a > > for & [BorrowedFormatItem < 'a >] { type Error = error :: DifferentVariant ; # [inline] fn try_from (value : BorrowedFormatItem < 'a >) -> Result < Self , Self :: Error > { match value { BorrowedFormatItem :: Compound (items) => Ok (items) , _ => Err (error :: DifferentVariant) , } } }
};
}

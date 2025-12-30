// Generated macro for impl_311 (impl)
macro_rules! Depcrate_format_description_owned_format_itemimpl_311 {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"impl_311"}
// Dependencies: {}
impl From < Vec < BorrowedFormatItem < '_ > > > for OwnedFormatItem { # [inline] fn from (items : Vec < BorrowedFormatItem < '_ > >) -> Self { items . as_slice () . into () } }
};
}

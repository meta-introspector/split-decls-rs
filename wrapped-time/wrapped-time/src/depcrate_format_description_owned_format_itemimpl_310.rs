// Generated macro for impl_310 (impl)
macro_rules! Depcrate_format_description_owned_format_itemimpl_310 {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"impl_310"}
// Dependencies: {}
impl From < & BorrowedFormatItem < '_ > > for OwnedFormatItem { # [inline] fn from (item : & BorrowedFormatItem < '_ >) -> Self { match item { BorrowedFormatItem :: Literal (literal) => { Self :: Literal (literal . to_vec () . into_boxed_slice ()) } BorrowedFormatItem :: Component (component) => Self :: Component (* component) , BorrowedFormatItem :: Compound (compound) => Self :: Compound (compound . iter () . cloned () . map (Into :: into) . collect :: < Vec < _ > > () . into_boxed_slice () ,) , BorrowedFormatItem :: Optional (item) => Self :: Optional (Box :: new ((* item) . into ())) , BorrowedFormatItem :: First (items) => Self :: First (items . iter () . cloned () . map (Into :: into) . collect :: < Vec < _ > > () . into_boxed_slice () ,) , } } }
};
}

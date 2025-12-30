// Generated macro for impl_312 (impl)
macro_rules! Depcrate_format_description_owned_format_itemimpl_312 {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"impl_312"}
// Dependencies: {}
impl < 'a , T : AsRef < [BorrowedFormatItem < 'a >] > + ? Sized > From < & T > for OwnedFormatItem { # [inline] fn from (items : & T) -> Self { Self :: Compound (items . as_ref () . iter () . cloned () . map (Into :: into) . collect :: < Vec < _ > > () . into_boxed_slice () ,) } }
};
}

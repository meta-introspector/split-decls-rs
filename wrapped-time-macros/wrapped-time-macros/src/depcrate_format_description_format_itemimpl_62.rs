// Generated macro for impl_62 (impl)
macro_rules! Depcrate_format_description_format_itemimpl_62 {
() => {
// Module: crate::format_description::format_item
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > From < Box < [Item < 'a >] > > for crate :: format_description :: public :: OwnedFormatItem { fn from (items : Box < [Item < 'a >] >) -> Self { let items = items . into_vec () ; match < [_ ; 1] > :: try_from (items) { Ok ([item]) => item . into () , Err (vec) => Self :: Compound (vec . into_iter () . map (Into :: into) . collect ()) , } } }
};
}

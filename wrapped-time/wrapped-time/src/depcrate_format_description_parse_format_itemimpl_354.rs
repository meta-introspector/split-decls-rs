// Generated macro for impl_354 (impl)
macro_rules! Depcrate_format_description_parse_format_itemimpl_354 {
() => {
// Module: crate::format_description::parse::format_item
// Provides: {"impl_354"}
// Dependencies: {}
impl < 'a > From < Box < [Item < 'a >] > > for crate :: format_description :: OwnedFormatItem { # [inline] fn from (items : Box < [Item < 'a >] >) -> Self { let items = items . into_vec () ; match < [_ ; 1] > :: try_from (items) { Ok ([item]) => item . into () , Err (vec) => Self :: Compound (vec . into_iter () . map (Into :: into) . collect ()) , } } }
};
}

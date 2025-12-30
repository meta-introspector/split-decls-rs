// Generated macro for impl_353 (impl)
macro_rules! Depcrate_format_description_parse_format_itemimpl_353 {
() => {
// Module: crate::format_description::parse::format_item
// Provides: {"impl_353"}
// Dependencies: {}
impl From < Item < '_ > > for crate :: format_description :: OwnedFormatItem { # [inline] fn from (item : Item < '_ >) -> Self { match item { Item :: Literal (literal) => Self :: Literal (literal . to_vec () . into_boxed_slice ()) , Item :: Component (component) => Self :: Component (component . into ()) , Item :: Optional { value , span : _ } => Self :: Optional (Box :: new (value . into ())) , Item :: First { value , span : _ } => { Self :: First (value . into_vec () . into_iter () . map (Into :: into) . collect ()) } } } }
};
}
